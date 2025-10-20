//! Basic interface for ERC20 tokens

use alloy::sol;

sol! {
    #[sol(rpc)]
    interface IERC20 {
        function approve(address spender, uint256 amount) external returns (bool);
    }
}
