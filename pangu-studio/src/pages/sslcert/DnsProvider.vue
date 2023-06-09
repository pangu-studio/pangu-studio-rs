<template>
    <el-row>
        <el-col v-for="provider in store.list" :key="provider.id" :xs="12" :sm="12" :md="8" :lg="6" :xl="4">
            <el-card :body-style="{ padding: '0px' }">
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
import {
    Setting
} from '@element-plus/icons-vue'

const store = useSslCertStore();

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


onMounted(() => {
    console.log("sslcert");
    store.listDnsProvider().then(() => {
        console.log(store.list);
    })
});
</script>
<style lang="scss" scoped>
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