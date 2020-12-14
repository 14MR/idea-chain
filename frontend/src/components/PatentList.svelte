<script>
    import ExpansionPanels, {
        ExpansionPanel,
    } from 'svelte-materialify/src/components/ExpansionPanels';
    import {eth} from '../eth.js';

    let about = {index: '', active: ''};


</script>

<ExpansionPanels>
    {#each eth.patents as patent}
        <ExpansionPanel>
            <span slot="header">{patent.agent_name}</span>
            {patent.owner}
        </ExpansionPanel>
    {/each}
</ExpansionPanels>