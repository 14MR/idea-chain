<script>
    import {
        AppBar,
        Button,
        Card,
        CardActions,
        CardText,
        CardTitle,
        Dialog,
        Tab,
        Tabs,
        Window,
        WindowItem
    } from 'svelte-materialify/src';
    import PatentList from "./PatentList.svelte";
    import Icon from 'svelte-materialify/src/components/Icon';
    import {mdiPlus} from '@mdi/js';
    import AddPatentForm from "./AddPatentForm.svelte";

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
            <PatentList/>
        </WindowItem>
    </Window>
</div>

<!--add button -->
<Button id="add-btn" fab size="default" class="blue white-text" on:click={handleClick}>
    <Icon path={mdiPlus}/>
</Button>
<Dialog persistent bind:active>
    <Card>
        <CardTitle>Add a contract</CardTitle>
        <CardText>
            <AddPatentForm></AddPatentForm>
        </CardText>
        <CardActions>
            <Button on:click={handleClick} text>Cancel</Button>
            <Button on:click={handleClick} text class="red-text">Save</Button>
        </CardActions>
    </Card>
</Dialog>