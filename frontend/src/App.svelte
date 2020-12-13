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

</script>


<MaterialApp theme="{theme}">
    <Menu/>
    <Container>
        <Row>
            <Col>
                {#if !$connected}
                    <span>loading...</span>
                {:else}
                    <Content/>
                {/if}
            </Col>
        </Row>
    </Container>
    token: {t}
    <hr>
    {checkAccount}
    <!--    Balance:-->
    <!--{#await balance}-->
    <!--    <span>waiting...</span>-->
    <!--{:then value}-->
    <!--    <span>{value}</span>-->
    <!--{/await}-->

    {#await connected}
        <span>waiting...</span>
    {:then value}
        <button on:click={sendAuth}>connect</button>
    {/await}
</MaterialApp>

<style>

</style>