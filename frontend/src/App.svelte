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
    import {abi} from "./abi.js";

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
            eth.patents = [];
            console.log(result[0]);
            console.log("smart contract")
            getPatents();

        })
        :
        ''

    const enableBrowser = () => ethStore.setBrowserProvider()

    async function sendAuth() {
        let signature = await $web3.eth.personal.sign(message, checkAccount);
        sendSignature(message, signature.slice(2));
    }

    let token = "";
    const unsubscribe = auth.subscribe(value => {
        token = value;
    });

    $: t = token


    function getPatents() {
        let contract = new $web3.eth.Contract(abi, '0xE87dbc35C7c4A446610bBa0F13b1a7eEf48a4117')
        contract.methods.contract_id().call().then(
            async el => {
                console.log("el", el)
                for (let i = 0; i <= el; i++) {
                    await contract.methods.getPatent(i).call().then(
                        c => {
                            console.log("before", c)
                            eth.update(e => {
                                e.patents[parseInt(c.id)] = c;
                                console.log(e.patents);
                                return e
                            });

                        }
                    )
                }


            }
        )

    }

    $: patents_view = $connected ? eth.patents : []

</script>


<MaterialApp theme="{theme}">
    <Menu/>
    <Container>
        <Row>
            <Col>
                {#if !$connected}
                    <span>waiting...</span>
                {:else if eth.patents}
                    <Content/>

                {/if}
            </Col>
        </Row>
    </Container>
</MaterialApp>

<style>

</style>
