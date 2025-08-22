
Parfait — voici le plan (implémentation visée en **Rust**) en **deux variantes** pour tester l’idée “branches parallèles / superposition” :

# A) Version pratique (CPU/GPU, dispo aujourd’hui)

## Idée

Un **seul modèle**, mais **chaque couche** possède **plusieurs versions de poids** (“branches”). À l’inférence, tu peux :

* les **pondérer** (soft mix),
* en **sélectionner k** (hard top-k),
* ou **exécuter plusieurs branches en parallèle** puis **choisir la meilleure** via un *vérifieur* → “**MoE sans MoE**”.

## Architecture (sans code)

* **Banque de versions par bloc** : `W = {W¹, W², …, W^m}` (m versions).
  Granularité au choix : bloc/attention head/MLP.
* **Gating contextuel** (léger) : encode l’input → vecteur `c`.
  Trois modes :

  1. **Soft mix** : `y = Σ α_i(c) · f(x; W^i)` (α via softmax).
  2. **Hard top-k** : ne calcule que les k plus probables.
  3. **Parallèle + vérifieur** : calcule p branches, **score** (coût/confiance/contraintes), garde 1 (ou en combine).
* **Vérifieur** (optionnel) : mesure la **pertinence** (ex : perplexité locale, alignement retrieval, règles symboliques satisfaites, cohérence temporelle).

## Apprentissage (continual learning natif)

* **Critère de conflit** (quand créer une nouvelle version) :

  * cosinus des gradients **négatif** ou Fisher élevé ⇒ la mise à jour **détruirait** du savoir ⇒ **spawn** `W^(m+1)` pour ce bloc (ou ajoute un **adapter** LoRA comme nouvelle “branche”).
* **Mise à jour** :

  * Si pas de conflit → fine-tune la branche active.
  * Si conflit → crée **nouvelle branche** et n’affecte pas les anciennes (évite l’oubli).
* **Régularisation** : EWC/SI/L2 sur anciennes branches pour limiter la dérive; **masques** ou **low-rank** pour factoriser les parts communes.
* **Compression/merge offline** : quand m devient grand, **fusionne** des branches proches (low-rank merge, distillation vers base + petits adapters).
* **Garbage-collect** : supprime branches rarement activées et à faible gain mesuré.

## Mémoire & contexte

* **Mémoire d’épisodes** + **index sémantique** (embeddings) gardent : (input, sortie, score, branche utilisée).
* Le gating apprend à **mapper le contexte** → **branche(s)** via ces traces.

## Coût & latence (maîtrise)

* Paramètres ↑ (m>1), latence maîtrisable par **top-k** + **budget de branches parallèles** (p).
* Le vérifieur peut être très léger (critères rapides) ou symbolique (contraintes du user).

---

# B) Version “quantique native” (vision hardware, demain)

## Mapping conceptuel

* Chaque “branche” = **composante d’un état quantique** ;
* Les “poids” = **paramètres d’unitaires** (rotations) empilés;
* L’apprentissage = ajuster ces paramètres (parameter-shift) **sans perte** (unitaires ≈ réversibles).
  **Sélection** = **mesure** de l’état → l’output “collabe” sur la branche pertinente.

## Atouts & obstacles

* **Atout théorique** : **pas d’oubli destructif** (on ajoute des rotations, on n’efface pas).
* **Obstacles** : peu de qubits, bruit, pas de QRAM pratique → irréaliste à grande échelle pour l’instant.

---

# Évaluation (pour valider vite que ça vaut le coup)

## Protocoles Continual Learning (multi-tâches/domaines)

* **Texte** : suites de tâches (intent → domain A/B/C), classification multi-domaines, code (bugs → fixes sur projets différents).
* **Vision** : Split CIFAR-100 / DomainNet (domain shift).
* **Multimodal** : VQA simple puis variantes par domaine.

## Metrics clés

* **Average Accuracy (AA)** en fin de séquence.
* **Forgetting (F)** par tâche (avant/après).
* **Backward/Forward Transfer (BWT/FWT)**.
* **Overhead compute** (×latence, ×params) vs. baseline.
* **Taux de spawn/merge** des branches, **usage par contexte**.

## Abladions indispensables

* Gating **soft vs hard vs parallèle+vérifieur**.
* **Granularité** des branches (bloc / tête / couche).
* **Seuil de conflit** (cos grad / Fisher).
* Avec/sans **compression** périodique.
* Vérifieur **statistique** vs **symbolique**.

---

# Risques & parades

* **Explosion du nb de branches** → impose **budget m max** + fusion périodique.
* **Gating instable** → ajouter entropie cible / load-balancing / label de domaine faible.
* **Biais vers anciennes branches** → exploration ε-greedy/top-p sur branches.
* **Coût mémoire** → factoriser via **adapters low-rank** plutôt que dupliquer des couches entières.

---

# Roadmap 10 jours (sans code, mais exécutable par n’importe qui)

1. **J1–J2** : baseline (modèle compact) + tâches séquentielles.
2. **J3–J4** : ajoute **branches par bloc** (m=2) + gating soft.
3. **J5** : **critère de conflit** (cos grad) ⇒ spawn/ freeze.
4. **J6** : **hard top-k** + vérifieur simple (perplexité/contrainte).
5. **J7** : **compression** (merge low-rank) + budget m.
6. **J8** : logs & métriques CL (AA, F, BWT, FWT).
7. **J9–J10** : ablations (granularité branches, seuils, p parallellisme) + rapport.

---

# Pourquoi ça vaut le coup

* **Apprentissage continu sans écraser** (les versions restent).
* **MoE-like intégré** mais **factorisé** (partage massif des poids).
* **Sélection émergente** (par contexte ou vérifieur), adaptable à **tout format**.

Si tu veux, je te fais un **tableau de décision** très court (3 lignes) pour choisir :

* *soft mix*, *hard top-k*, ou *parallèle+vérifieur*,
  en fonction de tes **contraintes de latence** et de **risque d’oubli**.
