

<template>
    <div class="markdown-body container" style="margin-top: 1rem">
<h1>TODO markdown page</h1>

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
    
    code {
        color: #000000 !important;
    }

    ul {
        list-style-type: disc !important;
    }
</style>
    
    