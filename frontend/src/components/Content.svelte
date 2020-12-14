<script>
    import {AppBar, Tab, Tabs, Window, WindowItem} from 'svelte-materialify/src';
    import PatentList from "./PatentList.svelte";
    import MyPatentList from "./MyPatentList.svelte";
    import AddBtn from "./AddBtn.svelte";

    let value = 0;
    let active = true;

    function handleClick() {
        active = !active;
    }
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
<div>
    <Window {value} class="ma-4">
        <WindowItem>
            <h4>All patents</h4>
            <div>
                <PatentList/>
            </div>
        </WindowItem>
        <WindowItem>
            <h4>My patents</h4>
            <MyPatentList/>
        </WindowItem>
    </Window>
</div>

<AddBtn/>