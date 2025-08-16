## Annexe 1. Mini flowchart ASCII (exécution Hi-MoE, sans bypass)

```
 +------------------+          +---------------------+
 |      Input       |          |     Orchestrator    |
 |  (InputData+ctx) |--------->| choose_router(x,ctx)|
 +------------------+          +---------+-----------+
                                         |
                                         v
                               +-----------------------+
                               |      Router_k         |
                               |  encode               |
                               |  gate                 |
                               |  pick top-k           |
                               |  call experts (par)   |
                               |  (collecte brute)     |
                               +----------+------------+
                                          |
                                          v
                               +-----------------------+
                               |   Orchestrator        |
                               |  synthétiser          |
                               |  (lit et transforme   |
                               |  les réponses experts |
                               |  en sortie exploitable)|
                               +----------+------------+
                                          |
                                          v
                               +----------+-----------+
                               |  Final answer (Value)|
                               +----------+-----------+
                                          |
                                          v
                    feedback -> routers/experts (no bypass)
```