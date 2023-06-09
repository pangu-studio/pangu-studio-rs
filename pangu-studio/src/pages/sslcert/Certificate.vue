<template>
    <el-table :data="tableData" style="width: 100%">
        <el-table-column label="域名" width="230">
            <template #default="scope">
                <el-tag class="domain-tag" v-for="domain in scope.row.domains.split(',')" type="success"
                    disable-transitions>{{ domain }}</el-tag>
            </template>
        </el-table-column>
        <el-table-column label="私钥" width="180">
            <template #default="scope">
                {{ handle_crt(scope.row.private_key) }}
                <icon-ic-baseline-content-copy style="cursor: pointer;" />
            </template>
        </el-table-column>
        <el-table-column label="证书" width="180">
            <template #default="scope">
                {{ handle_crt(scope.row.cert_chain) }}
                <icon-ic-baseline-content-copy style="cursor: pointer;" />
            </template>
        </el-table-column>
        <el-table-column prop="create_time" label="创建时间" />
    </el-table>
</template>

<script lang="ts" setup>
import { computed } from "vue";

const tableData = [
    {
        domains: 'www.pan.studio,pan.studio',
        private_key: 'xxx========\nxxxxxxxxxxxx',
        cert_chain: 'xxx========\nxxxxxxxxxxxx',
        create_time: '2016-05-03',
    },
    {
        domains: 'ab.pan.studio',
        private_key: 'xxx========\nxxxxxxxxxxxx',
        cert_chain: 'xxx========\nxxxxxxxxxxxx',
        create_time: '2016-05-03',
    },
]

const handle_crt = computed(() => {
    return (key_crt: string) => {
        return key_crt.substring(0, 3) + "********" + key_crt.substr(key_crt.length - 3, key_crt.length)
    }
})

</script>
<style lang="scss" scoped>
.domain-tag:not(:first-child) {
    margin-left: 8px;
}
</style>