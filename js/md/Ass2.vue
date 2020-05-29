

<template>
    <div class="markdown-body container" style="margin-top: 1rem">
<h1>hey</h1>

    </div>
</template>
    

<script>
    export default {
    mounted() {
        this.$nextTick(() => {
            hljs.initHighlighting();
        });
    },
    };
</script>
    

<style lang="scss">
    @import "@primer/css/markdown/index.scss";
</style>
    
    