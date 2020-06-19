

<template>
    <div class="markdown-body container" style="margin-top: 1rem">
<h2>Image Kernel to be used</h2>
<p>Write the image kernel you want to use in this demo in the following text box.</p>
<p>The syntax is as follows ––</p>
<ul>
<li>All numbers you enter should be floating point numbers, like so - <code>1.0</code>.</li>
<li>Arrays start and end with square brackets; individual elements are separated by commas, like so - <code>[a, b]</code>.</li>
<li>And an image kernel is ––
<ul>
<li>Either a python list of lists like - <code>[[1, 2], [3, 4]]</code></li>
<li>Or a list where the beginning of a new row is marked by a semicolon, like so - <code>[1, 2; 3, 4]</code></li>
</ul>
</li>
<li>There are a bunch of kernel matrices written in this format on a table near the end of this page for you to copy and try out. (like <code>[ [-1.0, -1.0, -1.0], [-1.0, 8.0, -1.0], [-1.0, -1.0, -1.0] ]</code>)</li>
</ul>

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
    
    