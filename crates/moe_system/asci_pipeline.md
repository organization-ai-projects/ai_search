## Annexe 1. Mini flowchart ASCII (exécution Hi-MoE, sans bypass)

```
+------------------+          +---------------------+
|      Input       |          |     Orchestrator    |
|  (InputData + ctx)|--------->| choose_router(x,ctx)|
+------------------+          +---------+-----------+
                                         |
                                         v
                               +---------+-----------+
                               |       Router_k      |
                               |  encode   |
                               |    ↓      |
                               |   gate    |
                               +----+-----------+----+
                                    |           |
                           top-k picks           |
                                    |           |
                                    v           |
                           +--------+--------+  |
                           |  call experts   |  |
                           | (parallelized)  |  |
                           +--------+--------+  |
                                    |           |
                                    v           v
                               +----+-----------+----+
                               |   aggregate (gates) |
                               | -> AggregatedOut    |
                               +----+-----------+----+
                                    |           |
                                    |     (optional)
                                    |     Shadow routers ...
                                    v
                             +------+-------+
                             |  Orchestrator|
                             |   synthesize |
                             +------+-------+
                                    |
                                    v
                         +----------+-----------+
                         |  Final answer (Value)|
                         +----------+-----------+
                                    |
                                    v
                    feedback -> routers/experts (no bypass)
```