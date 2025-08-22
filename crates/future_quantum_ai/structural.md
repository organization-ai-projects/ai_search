1) Architecture recommandée (alignée à ton but)

Cerveau unique versionné + types de cerveaux (par modalité/domaine).

Mémoire universelle :

Épisodique (journal d’interactions),

Vectorielle (RAG),

Graphe de connaissances (concepts/relations/contraintes).

Raisonnement explicite (symbolique/contraintes simples) pour vérifier/guider.


Branches parallèles dans les poids (par couche/bloc/tête, granularité adaptable) activées au contexte :

- Hard top-k (k=2) pour la latence,
- Option parallèle + vérifieur sur p=2 branches lorsque la confiance est basse.
- Possibilité de soft mix (pondération des branches via softmax sur le vecteur de contexte) pour certains blocs si besoin de flexibilité ou d’exploration.


Full vectoriel GPU : toutes les ops lourdes en tenseurs (matmul/attention/batched), indices en int32 (pas de usize à travers le compute).

2) Flux de données (pipeline)

Typage & contexte: encodage (modalité, domaine, contraintes).

Rappel mémoire: RAG (kNN) + sous-graphe pertinent → contexte enrichi.

Gating hiérarchique:

Étape A: choix type de cerveau (texte/vision/code/…)

Étape B: sélection de branches (top-2) dans les blocs concernés.

Exécution:

Mode par défaut: hard top-2 (GPU-friendly).

Si incertitude élevée → exécute p=2 branches en parallèle, passe au vérifieur.

Vérifieur (score composite):

Alignement RAG (cosine gain),

Contrainte symbolique (règles satisfaites/violées),

Confiance/entropie (proxy perplexité).
→ Garde 1 sortie ou combine pondérée.

Apprentissage online:

Log (entrée, sortie, score, branches utilisées).

Si update risque d’écraser → spawn nouvelle branche au lieu d’éditer l’ancienne.


Finetune périodique :
- Distillation vers la base + petits adapters,
- Merge/Compression de branches proches (ex : low-rank merge, distillation),
- GC des branches peu utiles.

3) Continual learning sans écrasement (règles simples)

Détection de conflit (quand créer une branche) :

cosinus(∇_nouveau, ∇_ancien) < –0.2 ou Fisher > τ ⇒ spawn.

Quotas pour éviter l’explosion :

m_max par bloc = 4 (au-delà → fusion/éjection),

budget p (branches parallèles) = 2 par requête max.

Régularisation :

EWC/SI sur anciennes branches,

Load-balancing du gating (évite l’effondrement sur 1 branche),

Exploration ε-greedy (ε ≈ 0.05) pour découvrir de nouvelles branches utiles.

4) Mémoire & raisonnement (ce qui rend “général”)

Épisodique: toutes les interactions + quelle branche a réussi.

Vectorielle: index multimodal (texte/image/audio/code) pour RAG.

Graphe: concepts, entités, relations, règles (ex: “ne jamais violer X”).

Raisonnement: petit moteur de contraintes (hard filters/soft penalties) branché dans le vérifieur → robustesse et explicabilité.

5) Paramètres par défaut (pour démarrer sans te perdre)

Gating: hiérarchique (Type → Branches), top-2, température 0.7.

Branches: par bloc MLP/attention seulement (pas partout).


Granularité : commence avec m=2 par bloc, m_max=4. Possibilité d’ajuster la granularité (bloc, tête, couche) selon les besoins ou les résultats des ablations.

Parallèle: active le mode p=2 + vérifieur uniquement si confiance < seuil.

Finetune: hebdo (ou quand assez d’épisodes), distillation vers base.

Merge: si distance < δ (p. ex. faible écart low-rank), sinon garde séparé.

GC: si une branche n’est pas choisie > N usages consécutifs et gain < γ.

6) Ce que ça t’apporte (au regard de ton objectif)

Un cerveau unique versionné et plusieurs cerveaux typés (modalités/domaines) sans duplication massive.

Apprentissage en continu sans écrasement destructif (on ajoute des versions).

MoE-like intégré mais factorisé (partage de poids, coût maîtrisé).

GPU-friendly (tout en tenseurs denses; les sélections via masques/gather).

Explicable (log: contexte → branches → règles → score du vérifieur).

7) Validation rapide (pour savoir si tu es sur la bonne voie)

Scénarios: enchaîner 3–4 domaines (ex: rédaction → code → vision → audio).

Mesures:

Forgetting par domaine (avant/après),

Average Accuracy, BWT/FWT,

Latence p95, coût compute,

Taux de spawn/merge/GC et usage des branches.

Succès = oubli quasi nul, perf ≥ baseline fine-tune, latence maîtrisée.

8) Plan d’attaque en 10 jours (sans code)

J1–J2: définir les types (modalités/domaines) + règles de vérifieur minimales.

J3: config mémoire (épisodique + vecteur + graphe) et features de contexte.

J4–J5: gating hiérarchique (top-2) + exécution full vectorielle.

J6: mode parallèle+p=2 déclenché sur faible confiance.

J7: heuristique spawn (cos∇/Fisher), quotas m_max.

J8: merge/distillation/GC (critères simples).

J9: protocole d’évaluation (scénarios + métriques).

J10: ablation (granularité branches, seuils, p, modes de gating soft/hard/parallèle, compression/merge) → fixes + doc. Prévoir des tests de variantes (ablation) pour valider l’impact de chaque choix d’architecture ou d’hyperparamètre.