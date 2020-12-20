// SPDX-License-Identifier: GPL-3.0

pragma solidity >=0.7.0 <0.8.0;
pragma experimental ABIEncoderV2;
contract Patent {

    uint public contract_id;
    struct PatentInfo {
        uint id;
        string inventor_name;
        address owner;
        string applicant_name;
        string agent_name;
        string state;
        string _address;
        string invention_title;
        string link;
        string country;
        uint patent_number_in_country;
        uint decision_number;
        uint decision_date;
        string law_number;
        string internation_classification_number;
        string person_responsible_in_country;
    }

    mapping(uint => PatentInfo) patents;
    uint[] public patentAccts;



    constructor() {
        contract_id = 0;
    }


    function changeOwner(uint contract_id, address newOwner) public {
        // abort if the caller is not an owner
        if (msg.sender !=  patents[contract_id].owner) return;

        patents[contract_id].owner = newOwner;
    }

    function addPatent(PatentInfo memory patent_info) public{


        patent_info.id = contract_id;
        patent_info.owner = msg.sender;
        patents[contract_id]= patent_info;

        patentAccts.push(contract_id);
        contract_id++;
    }

    function getPatent(uint patent_id) view public returns (PatentInfo memory){
        return patents[patent_id];
    }



}