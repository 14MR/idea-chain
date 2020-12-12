<script>
    import {Button, Card, CardActions, CardText, CardTitle, Dialog} from 'svelte-materialify/src';
    import Icon from 'svelte-materialify/src/components/Icon';
    import {mdiPlus} from '@mdi/js';
    import AddPatentForm from "./AddPatentForm.svelte";

    let value = 0;
    let active = false;

    function handleClick() {
        active = !active;
    }
</script>

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