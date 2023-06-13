<template>
    <el-card>
        <el-steps :active="stepActive" finish-status="success">
            <el-step title="提交证书申请" />
            <el-step title="验证域名" />
            <el-step title="签发证书" />
        </el-steps>
        <el-form :model="form" :rules="rules" ref="certFormRef" label-width="120px"
            :class="stepActive !== 1 ? 'hidden-form' : ''">
            <el-form-item label="签发机构">
                <span>Let's Encrypt</span>
            </el-form-item>

            <el-form-item label="证书有效期">
                <span>90天</span>
            </el-form-item>
            <el-form-item label="域名" prop="domain" required>
                <el-input v-model="form.domain" />
            </el-form-item>
            <el-form-item label="域名验证方式">
                <el-select v-model="form.check_type" placeholder="请选择">
                    <el-option label="自动DNS验证" value="dns" />
                    <!-- <el-option label="HTTP验证" value="http" /> -->
                </el-select>
            </el-form-item>
            <el-form-item label="DNS账号">
                <el-select v-model="form.provider_id" placeholder="请选择">
                    <el-option v-for="provider in store.list" :label="provider.name" :value="provider.id" />
                    <!-- <el-option label="HTTP验证" value="http" /> -->
                </el-select>
            </el-form-item>
            <el-form-item label="申请邮箱" prop="mail">
                <el-input v-model="form.mail" />
            </el-form-item>

            <el-form-item label="算法选择">
                <el-radio-group v-model="form.algorithm">
                    <el-radio label="ecc">ECC算法</el-radio>
                </el-radio-group>
            </el-form-item>

        </el-form>

        <div :class="stepActive !== 2 ? 'hidden-form' : ''">
            <div class="tip">已为您自动添加解析记录，请等待域名完成验证（可能需等待2分钟，请勿关闭程序!!!）</div>
            <el-table v-if="stepActive === 2" :data="store.applyCertAddtion">
                <el-table-column prop="identifier" label="主机记录" width="280" />
                <!-- <el-table-column prop="domain" label="域名" width="180" /> -->
                <el-table-column prop="record_type" label="记录类型" width="180" />
                <el-table-column prop="record_value" label="记录值" />
            </el-table>
        </div>
        <div :class="stepActive !== 3 ? 'hidden-form' : ''">
            <div class="tip">证书已签发，请前去列表查看</div>

            <el-table v-if="stepActive === 3" :data="store.applyCertArray" style="width: 100%">
                <el-table-column label="域名" width="230">
                    <template #default="scope">
                        <el-tag class="domain-tag" v-for="domain in scope.row.domains.split(',')" type="success"
                            disable-transitions>{{ domain }}</el-tag>
                    </template>
                </el-table-column>
                <el-table-column label="私钥" width="180">
                    <template #default="scope">
                        {{ handle_crt(scope.row.private_key) }}
                        <icon-ic-baseline-content-copy style="cursor: pointer;" @click="doCopy(scope.row.private_key)" />
                    </template>
                </el-table-column>
                <el-table-column label="证书" width="180">
                    <template #default="scope">
                        {{ handle_crt(scope.row.cert_chain) }}
                        <icon-ic-baseline-content-copy style="cursor: pointer;" @click="doCopy(scope.row.cert_chain)" />
                    </template>
                </el-table-column>
                <el-table-column label="状态" width="230">
                    <template #default="scope">
                        <el-tag class="domain-tag" v-if="scope.row.status === 'success'" type="success"
                            disable-transitions>已签发</el-tag>
                    </template>
                </el-table-column>
                <el-table-column label="创建时间">
                    <template #default="scope">
                        {{ datetimeFormat(scope.row.create_time) }}
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="operate-btn-group">
            <el-button v-loading.fullscreen.lock="fullscreenLoading" :class="stepActive !== 1 ? 'hidden-form' : ''"
                type="primary" @click="submmitApply(certFormRef)">提交证书申请</el-button>
            <el-button :class="stepActive !== 2 ? 'hidden-form' : ''" type="primary" loading>等待验证</el-button>
            <el-button type="" @click="router.push({ path: '/sslcert/certificates' })">返回列表</el-button>
        </div>
    </el-card>
</template>
<script lang="ts" setup>
import { ref, onMounted, reactive } from 'vue'
import { useSslCertStore } from '@/stores/sslcert'
import { FormRules, FormInstance } from 'element-plus'
import { SSLCertificateCreate, applyCertificate } from '@/api/sslcert'
import { useRouter } from "vue-router";
import useClipboard from 'vue-clipboard3'

import dateUtil from "@/utils/date"
const router = useRouter()
var store = useSslCertStore()

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

let stepActive = ref(1)
let form = reactive({
    domain: '',
    check_type: 'dns',
    provider_id: 1,
    mail: '',
    algorithm: 'ecc'
})
const certFormRef = ref<FormInstance>()
interface Addition {
    identifier: string,
    record_value: string,
    record_type: string
}

const fullscreenLoading = ref(false)


let addition = ref([] as Addition[])

// computed for date format
const datetimeFormat = computed(() => {
    return (date: string) => {
        return dateUtil.datetimeFormat(date)
    }
})

const handle_crt = computed(() => {
    return (key_crt: string) => {
        return key_crt.substring(0, 3) + "********" + key_crt.substr(key_crt.length - 3, key_crt.length)
    }
})
const rules = reactive<FormRules>({
    domain: [
        { required: true, message: '请输入域名', trigger: 'blur' },
        { pattern: /^([a-z0-9]+(-[a-z0-9]+)*\.)+([a-z0-9]+(-[a-z0-9]+)*\.)+[a-z]{2,}$/, message: '域名格式不正确,根域名申请请输入www子域名', trigger: 'blur' }
    ],
    mail: [
        { required: true, message: '请输入邮箱', trigger: 'blur' },
        { type: 'email', message: '邮箱格式不正确', trigger: 'blur' }
    ]
})

async function submmitApply(formEl: FormInstance | undefined) {
    if (!formEl) return
    await formEl.validate((valid, fields) => {
        if (valid) {
            console.log('submit!')
            fullscreenLoading.value = true
            const items = form.domain.split(".")
            const root = items[items.length - 2] + "." + items[items.length - 1]
            const subdomain = items.slice(0, items.length - 2).join(".")
            console.log(subdomain)
            console.log(root)
            console.log(form)

            let certCreate: SSLCertificateCreate = {
                sn: randomString(16),
                provider_id: form.provider_id,
                mail: form.mail,
                domain: root,
                subdomain: subdomain,
            }

            applyCertificate(certCreate).then(() => {
                console.log('apply success')
                stepActive.value = 3
                store.getSslCertBySN(certCreate.sn).then(() => {
                    console.log('fetch ssl cert success')
                }).catch(() => {
                    console.log('fetch ssl cert failed')
                    //todo
                })
            }).catch(() => {
                console.log('apply failed')
            })

            let timer = setInterval(() => {
                store.getSslCertBySN(certCreate.sn).then(() => {
                    console.log('fetch ssl cert success')
                    fullscreenLoading.value = false
                    stepActive.value = 2
                    clearInterval(timer)

                }).catch(() => {
                    console.log('fetch ssl cert failed')
                    //todo
                })
            }, 2000)
        } else {
            console.log('error submit!', fields)
        }
    })

}

function randomString(e: number) {
    e = e || 32;
    var t = "abcdefhijkmnprstwxyz2345678",
        a = t.length,
        n = "";
    for (let i = 0; i < e; i++) n += t.charAt(Math.floor(Math.random() * a));
    return n
}

onMounted(() => {
    // store.getSslCertBySN("3trzsafx5wx8sise").then(() => {
    //     console.log('fetch ssl cert list success')
    // })
    // stepActive.value = 3;
    store.listDnsProvider().then(() => {
        console.log('fetch dns provider list success')
    })
})

</script>
<style lang="scss" scoped>
// @use "element-plus/theme-chalk/src/dark/css-vars.scss" as *;

.operate-btn-group {
    display: flex;
    justify-content: flex-start;
    margin-top: 20px;
}

.tip {
    color: var(--el-text-color-primary);
    padding: 10px;
    font-size: 0.85em;
}

.hidden-form {
    display: none;
}
</style>