<template>
    <el-button type="primary" @click="showDialog = true">新增</el-button>
    <el-dialog v-model="showDialog" title="新增DNS账号">
        <el-form label-width="80">
            <el-form-item label="名称">
                <el-input v-model="providerForm.name"></el-input>
            </el-form-item>
            <el-form-item label="Key">
                <el-input v-model="providerForm.api_key"></el-input>
            </el-form-item>
            <el-form-item label="Secret">
                <el-input v-model="providerForm.api_secret"></el-input>
            </el-form-item>
            <el-form-item label="服务商">
                <el-radio-group v-model="providerForm.provider_type">
                    <el-radio label="dnspod">DNSPod</el-radio>
                </el-radio-group>
            </el-form-item>
        </el-form>
        <template #footer>
            <span class="dialog-footer">
                <el-button @click="showDialog = false">取消</el-button>
                <el-button type="primary" @click="handleSaveDnsProvider">
                    提交
                </el-button>
            </span>
        </template>

    </el-dialog>
    <el-row style="margin-top: 16px;">
        <el-col v-for="provider in store.list" :key="provider.id" :xs="12" :sm="12" :md="8" :lg="6" :xl="4">
            <el-card class="provider-card" :body-style="{ padding: '0px' }">
                <img :src="DnspodLogo" class="image" />
                <div style="padding: 14px;padding-top: 0;">
                    <span style="font-weight: 500;">{{ provider.name }}</span>
                    <div style="padding-top: 10px;">
                        <span>{{ provider.api_key }}</span>

                        <span style="float: right;">{{ handleSecret(provider.api_secret) }}</span>
                    </div>

                    <div class="bottom">
                        <time class="time">{{ datetimeFormat(provider.create_time) }}</time>
                        <el-button text class="button">设置</el-button>
                    </div>
                </div>
            </el-card>
        </el-col>
    </el-row>
</template>
<script lang="ts" setup>
import { onMounted, ref, computed } from "vue";
import { useSslCertStore } from "@/stores/sslcert";
import DnspodLogo from "@/assets/img/dnspod.png";
import moment from "moment";
import { CreateDnsProvider } from "@/api/sslcert";

const store = useSslCertStore();
const showDialog = ref(false);
let providerForm = ref({
    provider_type: "dnspod",
    name: "",
    api_key: "",
    api_secret: "",

} as CreateDnsProvider)
// computed for date format
const datetimeFormat = computed(() => {
    return (date: string) => {
        return moment(date).format('YYYY-MM-DD HH:mm:ss')
    }
})
const handleSecret = computed(() => {
    return (sk: string) => {
        return sk.substring(0, 3) + "********" + sk.substr(sk.length - 3, sk.length)
    }
})

const handleSaveDnsProvider = () => {
    store.addDnsProvider(providerForm.value).then(() => {
        showDialog.value = false;
    })
}
onMounted(() => {
    console.log("sslcert");
    store.listDnsProvider().then(() => {
        console.log(store.list);
    })
});
</script>
<style lang="scss" scoped>
.provider-card {
    font-size: 0.9em;
}

.time {
    font-size: 14px;
    color: #999;
}

.bottom {
    margin-top: 13px;
    line-height: 12px;
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.button {
    padding: 0 10px;
    min-height: auto;
}

.image {
    width: 100%;
    display: block;
}
</style>