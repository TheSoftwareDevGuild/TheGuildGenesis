source .env
forge script --chain amoy script/FullDeploymentScript.s.sol:FullDeploymentScript --rpc-url $AMOY_RPC_URL --broadcast --verify -vvvv --interactives 1
