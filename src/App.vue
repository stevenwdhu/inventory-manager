<template>
  <el-container>
    <el-aside style="width: 120px">
      <el-menu router :default-active="route.path">
        <el-menu-item index="/checkout"> 录入 </el-menu-item>
        <el-menu-item index="/inventory"> 库存 </el-menu-item>
        <el-menu-item index="/delivery_log"> 出库记录</el-menu-item>
        <el-menu-item index="/receiver_list">人员列表</el-menu-item>
        <el-menu-item index="/settings">设置</el-menu-item>
      </el-menu>
    </el-aside>
    <el-main>
      <router-view></router-view>
    </el-main>
  </el-container>
</template>

<script setup lang="ts">
import { useRoute, useRouter } from "vue-router";

// const router = useRouter();
const route = useRoute();
</script>

<style scoped>
.el-container {
  width: 100%;
  height: 100%;
}
.el-main {
  flex: 1;
  display: flex;
  height: 100%;
  justify-content: center;
}
</style>
