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
    $: checkAccount = $connected ? eth.account : '';
    // $: balance = $connected ? $web3.eth.getBalance(checkAccount) : ''
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


    function getPatents() {
        let contract = new $web3.eth.Contract(abi, '0xf8caa56A044b46f13655B2a07BbbCf1c94334981')
        contract.methods.contract_id().call().then(
            async el => {
                console.log("el", el)
                for (let i = 0; i <= el; i++) {
                    await contract.methods.getPatent(i).call().then(
                        c => {
                            eth.update(e => {e.patents.push(c); return e});
                            console.log(eth.patents)
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
    <hr>
    {checkAccount}

    {#await connected}
        <span>waiting...</span>
    {:then value}
        <button on:click={sendAuth}>connect</button>
    {/await}
</MaterialApp>

<style>

</style>
