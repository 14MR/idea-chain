<script>
    import {Tabs, Tab, Window, WindowItem, AppBar} from 'svelte-materialify/src';
    import PatentList from "./PatentList.svelte";

    let value = 0;
</script>

<AppBar>
    <span slot="title"> Patent registry </span>
    <div slot="extension">
        <Tabs class="green-text" bind:value fixedTabs>
            <div slot="tabs">
                <Tab>All patents</Tab>
                <Tab>My patents</Tab>
            </div>
        </Tabs>
    </div>
</AppBar>

<Window {value} class="ma-4">
    <WindowItem>
        <h4>All patents</h4>
        <div>
            <PatentList/>
        </div>
    </WindowItem>
    <WindowItem>
        <h4>My patents</h4>
        <PatentList/>
    </WindowItem>
</Window>