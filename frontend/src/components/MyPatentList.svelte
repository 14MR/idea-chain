<script>
    import ExpansionPanels, {
        ExpansionPanel,
    } from 'svelte-materialify/src/components/ExpansionPanels';
    import {eth} from '../eth.js';

    let about = {index: '', active: ''};
    import {auth} from '../auth';
    import {ListItem, Icon} from 'svelte-materialify/src';
</script>

<ExpansionPanels>
    {#each Object.values($eth.patents) as patent}
        {#if patent && patent.owner === auth.account}
            <ExpansionPanel>
                <span slot="header">{patent.agent_name}</span>
                <div class="d-flex justify-center mt-2 mb-2">
                    <div class="elevation-2">
                        {#each Object.keys(patent) as key}
                            {#if !Number.isInteger(parseInt(key))}
                                <ListItem><b>{key.replaceAll("_", " ")}:</b> {patent[key]}</ListItem>
                            {/if}
                        {/each}
                    </div>
                </div>
            </ExpansionPanel>
        {/if}
    {/each}
</ExpansionPanels>