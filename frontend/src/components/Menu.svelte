<script>
    import {mdiMenu, mdiViewDashboard, mdiAccountBox, mdiLogin, mdiLogout} from '@mdi/js';
    import {AppBar, Button, Icon, List, ListItem, NavigationDrawer, Overlay,} from 'svelte-materialify/src';
    import {connected, web3} from "svelte-web3";
    import {sendSignature} from "../api";
    import {eth} from "../eth";

    let active = false;

    $: checkAccount = $connected ? eth.account : '';
    const message = $web3.utils.sha3('test').slice(2);

    async function sendAuth() {
        let signature = await $web3.eth.personal.sign(message, checkAccount);
        sendSignature(message, signature.slice(2));
    }

    function toggleNavigation() {
        active = !active;
    }
</script>

<div>
    <AppBar>
        <div slot="icon">
            <Button fab depressed on:click={toggleNavigation}>
                <Icon path={mdiMenu}/>
            </Button>
        </div>
        <span slot="title"> Idea chain </span>
    </AppBar>
    <NavigationDrawer absolute {active}>
        <List>
            <ListItem>
        <span slot="prepend">
          <Icon path={mdiViewDashboard}/>
        </span>
                Dashboard
            </ListItem>
            <ListItem>
        <span slot="prepend">
          <Icon path={mdiAccountBox}/>
        </span>
                Account
            </ListItem>
            <ListItem on:click={sendAuth}>
        <span slot="prepend">
          <Icon path={mdiLogin}/>
        </span>
                Login
            </ListItem>
            <ListItem>
        <span slot="prepend">
          <Icon path={mdiLogout}/>
        </span>
                Logout            </ListItem>

        </List>
    </NavigationDrawer>
    <Overlay {active} absolute on:click={toggleNavigation} index={1}/>
</div>