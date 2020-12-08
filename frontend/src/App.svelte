<script>
	import { MaterialApp } from 'svelte-materialify';
	let theme = 'dark';
	import Menu from "./components/Menu.svelte";
	import { ethStore, web3, selectedAccount, connected } from 'svelte-web3';

	ethStore.setBrowserProvider();

	const enableBrowser = () => ethStore.setBrowserProvider()
	$: checkAccount = '0x077CA1590D6cf5222c92151c1a965C39ce08290B'
	$: balance = $connected ? $web3.eth.getBalance(checkAccount) : ''
</script>


<MaterialApp theme="{theme}">
	<Menu/>
	{checkAccount} Balance:
	{#await balance}
		<span>waiting...</span>
	{:then value}
		<span>{value}</span>
	{/await}
</MaterialApp>

<style>

</style>