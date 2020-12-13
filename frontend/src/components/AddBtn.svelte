<script>
    import {Button, Card, CardActions, CardText, CardTitle, Dialog} from 'svelte-materialify/src';
    import Icon from 'svelte-materialify/src/components/Icon';
    import {mdiPlus} from '@mdi/js';
    import TextField from 'svelte-materialify/src/components/TextField';
    import {Col, Row} from 'svelte-materialify/src/components/Grid';
    import {web3} from "svelte-web3";
    import {eth} from "../eth";

    let value = 0;
    let active = false;

    const abi = [
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

    let contract = new $web3.eth.Contract(abi, '0xb3F86c44859f1Ca393FC495C78b9008d0aeE7425')
    let patent_data = {
        id: 123,
        owner: '0x077CA1590D6cf5222c92151c1a965C39ce08290B',
        inventor_name: "",
        applicant_name: "",
        agent_name: '',
        state: '',
        _address: '',
        invention_title: '',
        link: '',
        country: '',
        patent_number_in_country: 0,
        decision_number: 0,
        decision_date: 0,
        law_number: 'No/1234',
        internation_classification_number: 'No/1245',
        person_responsible_in_country: "",
    };

    const handleSubmit = () => {
        active = !active;
        contract.methods.addPatent(patent_data).send({
            from: eth.account
        }).then(
            val => {
                console.log(val)
                contract.methods.getPatent(0).call().then(
                    val => console.log(val)
                )
            }
        );
    };


    const rules = [(v) => v.length <= 100 || 'Max 100 characters'];

    function handleClick() {
        active = !active;
    }
</script>

<Button id="add-btn" fab size="default" class="blue white-text" on:click={handleClick}>
    <Icon path={mdiPlus}/>
</Button>
<Dialog persistent bind:active>
    <Card>
        <CardTitle>Add a contract</CardTitle>
        <CardText>
            <form on:submit|preventDefault={handleSubmit}>
                <Col>
                    <Row>
                        <TextField clearable counter={100} {rules} value={patent_data.inventor_name}>inventor name
                        </TextField>
                    </Row>
                    <Row>
                        <TextField clearable counter={100} {rules} value={patent_data.applicant_name}>applicant name
                        </TextField>
                    </Row>
                    <Row>
                        <TextField clearable counter={100} {rules} value={patent_data.agent_name}>agent name</TextField>
                    </Row>
                    <Row>
                        <TextField clearable counter={100} {rules} value={patent_data.state}>state</TextField>
                    </Row>
                    <Row>
                        <TextField clearable counter={100} {rules} value={patent_data._address}>address</TextField>
                    </Row>
                    <Row>
                        <TextField clearable counter={100} {rules} value={patent_data.invention_title}>invention_title
                        </TextField>
                    </Row>
                    <Row>
                        <TextField clearable counter={100} {rules} value={patent_data.link}>link</TextField>
                    </Row>
                    <Row>
                        <TextField clearable counter={100} {rules} value={patent_data.country}>country</TextField>
                    </Row>
                    <Row>
                        <TextField clearable counter={100} {rules} value={patent_data.patent_number_in_country}>
                            patent number in country
                        </TextField>
                    </Row>
                    <Row>
                        <TextField clearable counter={100} {rules} value={patent_data.decision_number}>decision number
                        </TextField>
                    </Row>
                    <Row>
                        <TextField clearable counter={100} {rules} value={patent_data.decision_date}>decision date
                        </TextField>
                    </Row>
                    <Row>
                        <TextField clearable counter={100} {rules} value={patent_data.law_number}>law number</TextField>
                    </Row>
                    <Row>
                        <TextField clearable counter={100} {rules}
                                   value={patent_data.internation_classification_number}>
                            internation classification number
                        </TextField>
                    </Row>

                    <Row>
                        <TextField clearable counter={100} {rules} value={patent_data.person_responsible_in_country}>
                            person responsible in country
                        </TextField>
                    </Row>

                    <Button block type="submit">Submit</Button>
                </Col>
            </form>
        </CardText>
    </Card>
</Dialog>