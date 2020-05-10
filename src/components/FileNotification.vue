<template>
        <div class="notification is-primary notify">
            <div>Successfully parsed {{ this.$store.state.fileName }}
        </div>
</template>

<script>
export default {
    // computed: {
    // filePresent: function() {
    //   if (this.$store.state.fileParseSuccessful) {
       
    // },
    
}
</script>

<style scoped>
    .notify {
        padding: 0.8rem 0rem 0.8rem 0rem !important;
        margin-bottom: 0px !important;
        text-align: center;
    }
</style>