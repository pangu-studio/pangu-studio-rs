<template>
    <div class="main">
        <el-container style="min-height: 100%;">
            <el-aside class="sub-menu-aside" width="170" :collapse="isCollapse" :class="isCollapse ? activeClass : ''"
                :collapse-transition="false" style="min-height: 100%;display: flex;">
                <div class="sub-menu">

                    <router-link to="/sslcert/dnsprovider" style="text-decoration: none;">
                        <el-button type="" text bg><icon-carbon-server-dns style="margin-right: 4px;" />DNS服务商
                        </el-button>
                    </router-link>
                    <router-link to="/sslcert/certificates" style="text-decoration: none;">
                        <el-button type="" text bg><icon-carbon-certificate
                                style="margin-right: 4px" />证书管理&nbsp;&nbsp;&nbsp;&nbsp;
                        </el-button>
                    </router-link>
                    <router-link to="/sslcert/certificates" style="text-decoration: none;">
                        <el-button type="" text bg><icon-ri-pass-expired-line
                                style="margin-right: 4px" />证书监测&nbsp;&nbsp;&nbsp;&nbsp;
                        </el-button>
                    </router-link>

                </div>
                <div class="close-btn" style="" @click="isCollapse = !isCollapse">
                    <icon-ic:outline-keyboard-arrow-left class="arrow-btn" v-if="!isCollapse" />
                    <icon-ic-outline-keyboard-arrow-right class="arrow-btn" v-if="isCollapse" />
                </div>
            </el-aside>
            <el-container>
                <el-header class="header">

                    <el-breadcrumb :separator-icon="ArrowRight">
                        <el-breadcrumb-item :to="{ path: '/' }">首页</el-breadcrumb-item>
                        <el-breadcrumb-item>证书管理</el-breadcrumb-item>
                        <el-breadcrumb-item>{{ route.meta.title }}</el-breadcrumb-item>
                    </el-breadcrumb>
                </el-header>
                <el-main class="content"> <router-view /></el-main>
                <!-- <el-footer>Footer</el-footer> -->
            </el-container>
        </el-container>
    </div>
</template>
<script lang="ts" setup>
import { ref } from "vue";
import { ArrowRight } from '@element-plus/icons-vue'
import { useRoute } from 'vue-router';

const route = useRoute();

let activeClass = "sub-menu--collapse";

// const store = useSslCertStore();
const isCollapse = ref(false);
</script>
<style lang="scss" scoped>
.sub-menu--collapse {

    // width: 20px;
    .sub-menu {
        padding: 0;
        height: 100%;
        display: flex;
        width: 0;
    }
}

.sub-menu-aside {
    overflow: hidden;
}

.sub-menu-aside:not(.sub-menu--collapse) {
    .sub-menu {
        overflow: hidden;
        width: 140px;
        padding: 12px;

    }
}

.sub-menu {
    overflow: hidden;

    height: 100%;
    display: flex;
    // text-align: center;
    justify-content: center;
    // align-self: center;
    flex-direction: column;

    button {
        margin: 4px;
    }

}

.main {
    min-height: 100%;
    // margin: 15px;
}

.content {
    padding: 16px;

}

.close-btn {
    width: 8px;
    background-color: #edebeb;
    height: 100%;
    display: flex;
    margin-right: 0;
    justify-items: center;
    align-items: center;
    cursor: pointer;
}

.dark {
    .close-btn {
        background-color: #121212;
        color: #fff;
    }

}

.arrow-btn {
    // float: left;
    // z-index: 999;
    // margin-right: -15px;
    // height: auto;
    text-align: center;
    margin-right: -15px;
    margin-left: -13px;
    // color: #270303;


    width: 32px;
}

.header {
    height: 36px;
    padding: 16px;
}
</style>
