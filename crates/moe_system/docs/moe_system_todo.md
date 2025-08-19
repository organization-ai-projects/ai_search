# TODO & Checklist pour un MoE complet dans `moe_system`

## 1. Implémentations concrètes
- [ ] Orchestrator concret (struct qui orchestre tout le pipeline)
- [ ] Router concret (struct, même simple)
- [ ] Registre d’experts fonctionnel et initialisé
- [x] Synthétiseur par défaut (`DefaultSynthesizer`)
- [x] Traits et structures partagées (déjà présents)
- [x] Experts métiers de base (ex : NlpFrenchTagger, RulePlanner)

## 2. Chaîne d’appel complète
- [ ] Pipeline d’appel complet (main ou test d’intégration) : input → orchestrator → router → experts → synthétiseur → output
- [ ] Exemples d’utilisation de tous les composants ensemble

## 3. Gestion du feedback
- [x] Structures de feedback (`OrchestrationFeedback`, `RouterFeedback`)
- [ ] Logique concrète de gestion/propagation du feedback dans orchestrator et router

## 4. Tests et documentation
- [ ] Test(s) d’intégration (pipeline complet)
- [ ] README ou doc d’usage avec exemple de flux
- [x] Documentation technique (présente dans `docs/`)

## 5. Extensibilité
- [ ] Exemples d’extension (ajout d’un expert, d’un router, etc.)

---

## Points d’attention
- Tous les échanges entre modules doivent passer par des types partagés (`shared/`).
- Pas de dépendance métier croisée entre orchestrator, router, experts.
- Feedback et qualité : gérés côté orchestrator/registre, pas dans la sortie du router.
- Pipeline d’appel : doit être démontré dans un main ou un test d’intégration.

---

**Prochaines étapes recommandées** :
1. Implémenter un orchestrateur et un router concrets.
2. Créer un pipeline d’intégration minimal (test ou main).
3. Ajouter un exemple d’utilisation et de feedback.
4. Documenter le flux d’utilisation dans le README.
