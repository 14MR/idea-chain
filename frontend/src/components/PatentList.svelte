<script>
    import ExpansionPanels, {
        ExpansionPanel,
    } from 'svelte-materialify/src/components/ExpansionPanels';

    let patents = [
        {
            'title': "Some title",
            "description": "some desc"
        }
    ]

    let value = [1];
    let about = {index: '', active: ''};

    function onChange(e) {
        about = e.detail;
    }
</script>

<ExpansionPanels on:change={onChange} bind:value>
    {#each patents as patent}
        <ExpansionPanel>
            <span slot="header">{patent.title}</span>
            {patent.description}
        </ExpansionPanel>
    {/each}
</ExpansionPanels>