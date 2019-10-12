<template>
    <div v-on:click="runCode()" class = "executeButton">
        Exec ute
    </div>
</template>

<script>
export default {
    methods: {
        runCode() {
            this.$bus.emit('runCode');
        }
    }
}
</script>
<style scoped>
.executeButton {
    height:35px; 
    width:35px; 
    color:white; 
    background:green;
    cursor: pointer;
}
</style>