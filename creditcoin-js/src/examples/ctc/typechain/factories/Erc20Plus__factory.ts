/* Autogenerated file. Do not edit manually. */
/* tslint:disable */
/* eslint-disable */

import { Contract, Signer, utils } from 'ethers';
import { Provider } from '@ethersproject/providers';
import type { Erc20Plus, Erc20PlusInterface } from '../Erc20Plus';

const _abi = [
    {
        constant: false,
        inputs: [
            {
                name: 'spender',
                type: 'address',
            },
            {
                name: 'value',
                type: 'uint256',
            },
        ],
        name: 'approve',
        outputs: [
            {
                name: 'success',
                type: 'bool',
            },
        ],
        payable: false,
        stateMutability: 'nonpayable',
        type: 'function',
    },
    {
        constant: true,
        inputs: [],
        name: 'totalSupply',
        outputs: [
            {
                name: 'amount',
                type: 'uint256',
            },
        ],
        payable: false,
        stateMutability: 'view',
        type: 'function',
    },
    {
        constant: false,
        inputs: [
            {
                name: 'from',
                type: 'address',
            },
            {
                name: 'to',
                type: 'address',
            },
            {
                name: 'value',
                type: 'uint256',
            },
        ],
        name: 'transferFrom',
        outputs: [
            {
                name: 'success',
                type: 'bool',
            },
        ],
        payable: false,
        stateMutability: 'nonpayable',
        type: 'function',
    },
    {
        constant: false,
        inputs: [
            {
                name: 'value',
                type: 'uint256',
            },
        ],
        name: 'burn',
        outputs: [
            {
                name: 'success',
                type: 'bool',
            },
        ],
        payable: false,
        stateMutability: 'nonpayable',
        type: 'function',
    },
    {
        constant: true,
        inputs: [
            {
                name: 'owner',
                type: 'address',
            },
        ],
        name: 'balanceOf',
        outputs: [
            {
                name: 'balance',
                type: 'uint256',
            },
        ],
        payable: false,
        stateMutability: 'view',
        type: 'function',
    },
    {
        constant: false,
        inputs: [
            {
                name: 'from',
                type: 'address',
            },
            {
                name: 'value',
                type: 'uint256',
            },
        ],
        name: 'burnFrom',
        outputs: [
            {
                name: 'success',
                type: 'bool',
            },
        ],
        payable: false,
        stateMutability: 'nonpayable',
        type: 'function',
    },
    {
        constant: false,
        inputs: [
            {
                name: 'to',
                type: 'address',
            },
            {
                name: 'value',
                type: 'uint256',
            },
        ],
        name: 'transfer',
        outputs: [
            {
                name: 'success',
                type: 'bool',
            },
        ],
        payable: false,
        stateMutability: 'nonpayable',
        type: 'function',
    },
    {
        constant: true,
        inputs: [
            {
                name: 'owner',
                type: 'address',
            },
            {
                name: 'spender',
                type: 'address',
            },
        ],
        name: 'allowance',
        outputs: [
            {
                name: 'remaining',
                type: 'uint256',
            },
        ],
        payable: false,
        stateMutability: 'view',
        type: 'function',
    },
    {
        anonymous: false,
        inputs: [
            {
                indexed: true,
                name: 'from',
                type: 'address',
            },
            {
                indexed: false,
                name: 'value',
                type: 'uint256',
            },
        ],
        name: 'Burnt',
        type: 'event',
    },
    {
        anonymous: false,
        inputs: [
            {
                indexed: true,
                name: 'from',
                type: 'address',
            },
            {
                indexed: true,
                name: 'to',
                type: 'address',
            },
            {
                indexed: false,
                name: 'value',
                type: 'uint256',
            },
        ],
        name: 'Transfer',
        type: 'event',
    },
    {
        anonymous: false,
        inputs: [
            {
                indexed: true,
                name: 'owner',
                type: 'address',
            },
            {
                indexed: true,
                name: 'spender',
                type: 'address',
            },
            {
                indexed: false,
                name: 'value',
                type: 'uint256',
            },
        ],
        name: 'Approval',
        type: 'event',
    },
];

export class Erc20Plus__factory {
    static readonly abi = _abi;
    static createInterface(): Erc20PlusInterface {
        return new utils.Interface(_abi) as Erc20PlusInterface;
    }
    static connect(address: string, signerOrProvider: Signer | Provider): Erc20Plus {
        return new Contract(address, _abi, signerOrProvider) as Erc20Plus;
    }
}
