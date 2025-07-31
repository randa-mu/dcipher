# OnlySwaps-verifier

The OnlySwaps verifier is a dcipher agent that:

    - listens to the `Router` contract on each supported chain for a `BridgeReceipt` event
    - when the `BridgeReceipt` event is emitted, it verifies its values against the `transferParameters` on the `sourceChain`
    - if the values match, requests a `dsigner` (partial) signature attesting to that
    - once a full group signature is achieved, execute a transaction on the source chain to `rebalanceSolver` using the group signature

Furthermore, it:
    - can be configured to listen across many chains
    - can be configured to dry-run transactions to rebalanceSolver rather than execute them (similar to blocklock-agent)

