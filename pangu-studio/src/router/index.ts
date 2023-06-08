import { createWebHistory, createRouter } from "vue-router";
import Container from "~/pages/container/index.vue";
import SslCert from "~/pages/sslcert/index.vue";
import Database from "~/pages/database/index.vue";
import AppStore from "~/pages/appstore/index.vue";
import Setting from "~/pages/setting/index.vue";
import FileManager from "~/pages/filemanager/index.vue";
import { appWindow } from '@tauri-apps/api/window';

const routes = [
    {
        path: "/",
        name: "Home",
        component: Container,
        meta: {
            title: "容器管理"
        }
    },
    {
        path: "/sslcert",
        name: "SslCert",
        component: SslCert,
        meta: {
            title: "证书管理"
        }
    },
    {
        path: "/database",
        name: "Database",
        component: Database,
        meta: {
            title: "数据库管理"
        }
    },
    {
        path: "/filemanager",
        name: "FileManager",
        component: FileManager,
        meta: {
            title: "文件管理器"
        }
    },
    {
        path: "/appstore",
        name: "AppStore",
        component: AppStore,
        meta: {
            title: "应用商店"
        }
    },
    {
        path: "/setting",
        name: "Setting",
        component: Setting,
        meta: {
            title: "设置"
        }
    },
];

const router = createRouter({
    history: createWebHistory(),
    routes,
});
const defaultTitle = 'Pangu Studio'
router.beforeEach((to, from, next) => {
 const title = (to.meta.title ? to.meta.title : defaultTitle) as string
  appWindow.setTitle(title)
  console.log(title)
  next()
})

export default router;