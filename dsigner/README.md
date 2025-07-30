# dsigner
A pre-packaged instance of `dsigner` running with `docker` is provided in [`./test-server`](./test-server).
This instance uses the BN254 curve with signatures on G1.
It is configured with a threshold of two, and three different nodes, running on port 8080, 8081, and 8082.

The configuration can be modified by updating the configuration in [`./test-server`](./test-server).

To start a test instance of `dsigner`, simply execute:
```bash
> docker compose -f ./test-server/docker-compose.yml up
```

***Fetching the `dsigner` public key***  
```bash
> curl http://127.0.0.1:8080/pk
"09537fc85a65d2b508ac5788838546236ff88eacd3742990a6759eb91b2d73361d98288af7e2ba6ca03103a7acb95821e376b3454b739d9c7432651ca8c665b216462e9d7fce8f0775115d6b063a08876bc2f21de6268f88296bde7a7c5cd815275ac7684fb4c7d2f41ae4ff4b7c0787b35e0fd115dc1e97cdc9adedd0778546"
```

`dsigner` returns the public key on G2 encoded as `BE(x_1) || BE(x_0) || BE(y_1) || BE(y_0)` where $pk = (x_1 \cdot i + x_0,\  y_1 \cdot i + y_0)$.

***Requesting a signature***
```bash
curl -X POST http://127.0.0.1:8080/sign -H 'Content-Type: application/json' -d '{"m": "Hello World"}' -s
{"signature":"ln248jbQMEcdjZTBYN/G2MMSlyWxylEFJzjwla2q7Bs=","dst":"dsigner-v01-BN254G1_XMD:KECCAK-256_SVDW_RO_"}
```
