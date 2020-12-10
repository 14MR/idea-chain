<script>
	import { MaterialApp } from 'svelte-materialify';
	let theme = 'dark';
	import Menu from "./components/Menu.svelte";
	import {sendSignature} from './api';
	import {auth} from './auth';
	import { ethStore, web3, selectedAccount, connected } from 'svelte-web3';

	ethStore.setBrowserProvider();

	const enableBrowser = () => ethStore.setBrowserProvider()
	$: checkAccount = '0x077CA1590D6cf5222c92151c1a965C39ce08290B'
	$: balance = $connected ? $web3.eth.getBalance(checkAccount) : ''
	const message = $web3.utils.sha3('test').slice(2);

	async function sendAuth(){
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
	token: {t} <hr>
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