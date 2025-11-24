//SPDX-License-Identifier: MIT

pragma solidity ^0.8.20;

import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {Fund} from "./Fund.sol";

contract FundTokenERC20 is ERC20 {
    Fund fund;
    constructor(address fundAddr) ERC20("FundTokenERC20", "FT") {
        fund = Fund(fundAddr);
    }

    function mint(uint256 amountToMint) public {
        require(fund.fundersToAmount(msg.sender) >= amountToMint, "You cannot mint this many tokens");
        require(fund.getFundSuccess(), "The fundme is not completed yet");
        _mint(msg.sender, amountToMint);
        fund.setFunderToAmount(msg.sender, fund.fundersToAmount(msg.sender) - amountToMint);
    }

    function claim(uint256 amountToClaim) public {
        // complete cliam
        require(balanceOf(msg.sender) >= amountToClaim, "You dont have enough ERC20 tokens");
        require(fund.getFundSuccess(), "The fundme is not completed yet");
        /*to add */
        // burn amountToClaim Tokens       
        _burn(msg.sender, amountToClaim);
    }
}