export const abi =[
    {
        "inputs": [],
        "stateMutability": "nonpayable",
        "type": "constructor"
    },
    {
        "inputs": [
            {
                "components": [
                    {
                        "internalType": "uint256",
                        "name": "id",
                        "type": "uint256"
                    },
                    {
                        "internalType": "string",
                        "name": "inventor_name",
                        "type": "string"
                    },
                    {
                        "internalType": "address",
                        "name": "owner",
                        "type": "address"
                    },
                    {
                        "internalType": "string",
                        "name": "applicant_name",
                        "type": "string"
                    },
                    {
                        "internalType": "string",
                        "name": "agent_name",
                        "type": "string"
                    },
                    {
                        "internalType": "string",
                        "name": "state",
                        "type": "string"
                    },
                    {
                        "internalType": "string",
                        "name": "_address",
                        "type": "string"
                    },
                    {
                        "internalType": "string",
                        "name": "invention_title",
                        "type": "string"
                    },
                    {
                        "internalType": "string",
                        "name": "link",
                        "type": "string"
                    },
                    {
                        "internalType": "string",
                        "name": "country",
                        "type": "string"
                    },
                    {
                        "internalType": "uint256",
                        "name": "patent_number_in_country",
                        "type": "uint256"
                    },
                    {
                        "internalType": "uint256",
                        "name": "decision_number",
                        "type": "uint256"
                    },
                    {
                        "internalType": "uint256",
                        "name": "decision_date",
                        "type": "uint256"
                    },
                    {
                        "internalType": "string",
                        "name": "law_number",
                        "type": "string"
                    },
                    {
                        "internalType": "string",
                        "name": "internation_classification_number",
                        "type": "string"
                    },
                    {
                        "internalType": "string",
                        "name": "person_responsible_in_country",
                        "type": "string"
                    }
                ],
                "internalType": "struct Patent.PatentInfo",
                "name": "patent_info",
                "type": "tuple"
            }
        ],
        "name": "addPatent",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
    },
    {
        "inputs": [
            {
                "internalType": "uint256",
                "name": "contract_id",
                "type": "uint256"
            },
            {
                "internalType": "address",
                "name": "newOwner",
                "type": "address"
            }
        ],
        "name": "changeOwner",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
    },
    {
        "inputs": [],
        "name": "contract_id",
        "outputs": [
            {
                "internalType": "uint256",
                "name": "",
                "type": "uint256"
            }
        ],
        "stateMutability": "view",
        "type": "function"
    },
    {
        "inputs": [
            {
                "internalType": "uint256",
                "name": "patent_id",
                "type": "uint256"
            }
        ],
        "name": "getPatent",
        "outputs": [
            {
                "components": [
                    {
                        "internalType": "uint256",
                        "name": "id",
                        "type": "uint256"
                    },
                    {
                        "internalType": "string",
                        "name": "inventor_name",
                        "type": "string"
                    },
                    {
                        "internalType": "address",
                        "name": "owner",
                        "type": "address"
                    },
                    {
                        "internalType": "string",
                        "name": "applicant_name",
                        "type": "string"
                    },
                    {
                        "internalType": "string",
                        "name": "agent_name",
                        "type": "string"
                    },
                    {
                        "internalType": "string",
                        "name": "state",
                        "type": "string"
                    },
                    {
                        "internalType": "string",
                        "name": "_address",
                        "type": "string"
                    },
                    {
                        "internalType": "string",
                        "name": "invention_title",
                        "type": "string"
                    },
                    {
                        "internalType": "string",
                        "name": "link",
                        "type": "string"
                    },
                    {
                        "internalType": "string",
                        "name": "country",
                        "type": "string"
                    },
                    {
                        "internalType": "uint256",
                        "name": "patent_number_in_country",
                        "type": "uint256"
                    },
                    {
                        "internalType": "uint256",
                        "name": "decision_number",
                        "type": "uint256"
                    },
                    {
                        "internalType": "uint256",
                        "name": "decision_date",
                        "type": "uint256"
                    },
                    {
                        "internalType": "string",
                        "name": "law_number",
                        "type": "string"
                    },
                    {
                        "internalType": "string",
                        "name": "internation_classification_number",
                        "type": "string"
                    },
                    {
                        "internalType": "string",
                        "name": "person_responsible_in_country",
                        "type": "string"
                    }
                ],
                "internalType": "struct Patent.PatentInfo",
                "name": "",
                "type": "tuple"
            }
        ],
        "stateMutability": "view",
        "type": "function"
    },
    {
        "inputs": [
            {
                "internalType": "uint256",
                "name": "",
                "type": "uint256"
            }
        ],
        "name": "patentAccts",
        "outputs": [
            {
                "internalType": "uint256",
                "name": "",
                "type": "uint256"
            }
        ],
        "stateMutability": "view",
        "type": "function"
    }
];