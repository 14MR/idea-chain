<script>
    import {MaterialApp} from 'svelte-materialify';

    let theme = 'dark';
    import Menu from "./components/Menu.svelte";
    import {Container, Row, Col} from 'svelte-materialify/src';
    import {sendSignature} from './api';
    import {auth} from './auth';
    import {eth} from './eth';
    import {ethStore, web3, selectedAccount, connected} from 'svelte-web3';
    import PatentList from "./components/PatentList.svelte";
    import Content from "./components/Content.svelte";

    const abi =[
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
            "stateMutability": "nonpayable",
            "type": "function"
        }
    ];
    ethStore.setBrowserProvider();

    function getAccounts(callback) {
        $web3.eth.getAccounts((error, result) => {
            if (error) {
                console.log(error);
            } else {
                callback(result);
            }
        });
    }

    $: smart_contract_interface = $connected ? getAccounts((result) => {
            eth.account = result[0];
            console.log(result[0]);
            console.log("smart contract")
            let contract = new $web3.eth.Contract(abi, '0xb3F86c44859f1Ca393FC495C78b9008d0aeE7425')
            console.log(contract)
            contract.methods.addPatent({
                id: 123,
                owner: '0x077CA1590D6cf5222c92151c1a965C39ce08290B',
                inventor_name: "some name",
                applicant_name: "applicant_name",
                agent_name: 'agent_name',
                state: 'state',
                _address: '_address',
                invention_title: 'invention_title',
                link: 'link',
                country: 'country',
                patent_number_in_country: 1,
                decision_number: 123,
                decision_date: 124,
                law_number: 'sdsd/1234',
                internation_classification_number: 'No/1245',
                person_responsible_in_country: "Mister Ben",
            }).send({
                from: eth.account
            }).then(
                val => {
                    console.log(val)
                    contract.methods.getPatent(0).call().then(
                        val => console.log(val)
                    )
                }
            );



        })
        :
        ''

    const enableBrowser = () => ethStore.setBrowserProvider()
    $: checkAccount = eth.account
    $: balance = $connected ? $web3.eth.getBalance(checkAccount) : ''
    const message = $web3.utils.sha3('test').slice(2);

    async function sendAuth() {
        let signature = await $web3.eth.personal.sign(message, checkAccount);
        sendSignature(message, signature.slice(2));
    }

    let token = "";
    const unsubscribe = auth.subscribe(value => {
        token = value;
    });

    $: t = token

</script>


<MaterialApp theme="{theme}">
    <Menu/>
    <Container>
        <Row>
            <Col>
                <Content/>
            </Col>
        </Row>
    </Container>
    token: {t}
    <hr>
    {checkAccount} Balance:
    {#await balance}
        <span>waiting...</span>
    {:then value}
        <span>{value}</span>
    {/await}

    {#await connected}
        <span>waiting...</span>
    {:then value}
        <button on:click={sendAuth}>connect</button>
    {/await}
</MaterialApp>

<style>

</style>