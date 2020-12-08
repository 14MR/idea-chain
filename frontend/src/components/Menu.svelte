<script>
    import {mdiMenu, mdiViewDashboard, mdiAccountBox, mdiLogin, mdiLogout} from '@mdi/js';
    import {AppBar, Button, Icon, List, ListItem, NavigationDrawer, Overlay,} from 'svelte-materialify/src';

    let active = false;

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
            <ListItem>
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