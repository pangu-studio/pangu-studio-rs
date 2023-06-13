<template>
    <div class="toolbar">
        <el-button type="primary" @click="toApplyCertPage">申请免费证书</el-button>
    </div>

    <el-table :data="store.sslcerts" style="width: 100%" row-key="id">
        <el-table-column label="域名" width="200">
            <template #default="scope">
                <el-tag class="domain-tag" v-for="domain in scope.row.domains.split(',')" type="success"
                    disable-transitions>{{ domain }}</el-tag>
            </template>
        </el-table-column>
        <el-table-column label="私钥">
            <template #default="scope">
                {{ handle_crt(scope.row.private_key) }}
                <icon-ic-baseline-content-copy style="cursor: pointer;" @click="doCopy(scope.row.private_key)" />
            </template>
        </el-table-column>
        <el-table-column label="证书">
            <template #default="scope">
                {{ handle_crt(scope.row.cert_chain) }}
                <icon-ic-baseline-content-copy style="cursor: pointer;" @click="doCopy(scope.row.cert_chain)" />
            </template>
        </el-table-column>
        <el-table-column label="状态" width="130">
            <template #default="scope">
                <el-tag class="domain-tag" v-if="scope.row.status === 'success'" type="success"
                    disable-transitions>已签发</el-tag>
            </template>
        </el-table-column>
        <el-table-column label="过期日期" width="120">
            <template #default="scope">
                {{ expireTime(scope.row.create_time) }}
            </template>
        </el-table-column>
        <el-table-column label="创建时间" width="180">
            <template #default="scope">
                {{ datetimeFormat(scope.row.create_time) }}
            </template>
        </el-table-column>
        <el-table-column fixed="right" label="操作" width="140">
            <template #default="scope">
                <el-button link type="primary" size="small" @click="renew">重新申请</el-button>
                <el-button link type="danger" size="small" @click="removeCert(scope.row.id)">删除</el-button>
            </template>
        </el-table-column>

    </el-table>
    <el-dialog v-model="dialogVisible" title="温馨提示" width="30%" :before-close="handleClose">
        <span>确定将该证书移到回收站?</span>
        <template #footer>
            <span class="dialog-footer">
                <el-button @click="dialogVisible = false">取消</el-button>
                <el-button type="primary" @click="handleRemoveCert">
                    确认
                </el-button>
            </span>
        </template>
    </el-dialog>
</template>

<script lang="ts" setup>
import { computed, onMounted } from "vue";
import { useSslCertStore } from "~/stores/sslcert"
import { useRouter } from "vue-router"
import dateUtil from "@/utils/date"
import moment from "moment"
import useClipboard from 'vue-clipboard3'
import { ElMessage } from "element-plus"
import { ElMessageBox } from 'element-plus'

const dialogVisible = ref(false)

const store = useSslCertStore()
const router = useRouter()
const { toClipboard } = useClipboard()

const copyMessage = "复制成功"
const doCopy = async (txt: string) => {

    try {
        await toClipboard(txt).then(() => {
            ElMessage({
                message: copyMessage,
                type: 'success',
            })
        })
    } catch (e) {
        console.error(e)
    }
}
const renew = () => {
    alert('即将上线')

}
// computed for date format
const datetimeFormat = computed(() => {
    return (date: string) => {
        return dateUtil.datetimeFormat(date)
    }
})
const expireTime = computed(() => {
    return (date: string) => {
        const ti = moment(date);
        const exd = ti.add(90, 'days').format('YYYY-MM-DD');
        return exd
    }
})
const handle_crt = computed(() => {
    return (key_crt: string) => {
        return key_crt.substring(0, 3) + "********" + key_crt.substr(key_crt.length - 3, key_crt.length)
    }
})
function toApplyCertPage() {
    console.log('toApplyCertPage')
    router.push({
        path: '/sslcert/apply'
    })
}
let removeCertId = ref(0)
const handleRemoveCert = () => {
    console.log('handleRemoveCert', removeCertId.value)
    store.removeSslCert(removeCertId.value).then(() => {
        console.log('remove cert success')
        dialogVisible.value = false
    })
}
const removeCert = (certId: number) => {
    removeCertId.value = certId
    dialogVisible.value = true
    console.log('removeCert', certId)
}
const handleClose = (done: () => void) => {
    ElMessageBox.confirm('确定关闭对话框?')
        .then(() => {
            done()
        })
        .catch(() => {
            // catch error
            console.log('catch error')
        })
}

onMounted(() => {
    console.log('onMounted')

    store.listSSLCertificate().then(() => {
        // console.log(store.sslcerts)
    })

})
</script>
<style lang="scss" scoped>
.toolbar {
    margin-bottom: 20px;
}

.domain-tag:not(:first-child) {
    margin-left: 8px;
}
</style>