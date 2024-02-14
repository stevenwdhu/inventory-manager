<template>
  <el-container>
    <el-dialog
      v-model="modalVisible"
      title="更改姓名"
      width="500px"
      @open="
        () => {
          tmpDepartment = '';
          tmpName = '';
        }
      "
    >
      <el-form style="width: 300px; padding-left: 50px">
        <el-form-item label="部门">
          <el-select
            style="width: 200px"
            v-model="tmpDepartment"
            allow-create
            filterable
            default-first-option
            :reserve-keyword="false"
            placeholder="请选择或输入部门"
          >
            <el-option
              v-for="d in departmentList"
              :key="d"
              :label="d"
              :value="d"
            />
          </el-select>
        </el-form-item>
        <el-form-item label="姓名">
          <el-input
            v-focus
            v-model="tmpName"
            placeholder="请输入姓名"
            @keyup.enter="handleNewPerson"
          />
        </el-form-item>
      </el-form>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="modalVisible = false">Cancel</el-button>
          <el-button type="primary" @click="handleNewPerson">
            Confirm
          </el-button>
        </span>
      </template>
    </el-dialog>
    <header>
      <el-button type="success" @click="() => (modalVisible = true)">
        新增人员
      </el-button>
    </header>
    <el-table :data="receiverList" border style="width: 100%; flex: 1">
      <el-table-column
        type="index"
        :index="(i:number)=>i+1"
        label="序号"
        align="center"
        width="80px"
      />
      <el-table-column label="部门" prop="department" align="center" />
      <el-table-column label="姓名" prop="name" align="center" />
      <el-table-column fixed="right" label="操作" width="80" align="center">
        <template #default="scope">
          <el-button
            link
            type="danger"
            size="small"
            @click.prevent="handleDelete(scope.$index)"
            >删除
          </el-button>
        </template>
      </el-table-column>
    </el-table>
  </el-container>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import { IReceiver } from "../types";
import { invoke } from "@tauri-apps/api/tauri";
import { RECEIVER_LIST } from "../const";
import { ElMessage } from "element-plus";

// const receiverList = ref({
//   中心领导: [],
//   综合业务部: ["胡文棣", "徐启梅", "张雅静", "许旭彦", "吴慧玉", "吴晓彤"],
//   复核部: ["李建勇", "彭博", "李聂琼", "杨文燕"],
//   房建装饰部: [],
//   市政园林部: [],
//   设备安装部: [],
//   公路水利部: [],
// });
const modalVisible = ref(false);
const tmpDepartment = ref("");
const tmpName = ref("");

const handleNewPerson = () => {
  receiverList.value.push({
    department: tmpDepartment.value,
    name: tmpName.value,
  });
  modalVisible.value = false;
};

const receiverList = ref<IReceiver[]>([]);

const departmentList = computed(() => {
  return new Set(receiverList.value.map((p) => p.department));
});

const handleDelete = (idx: number) => {
  receiverList.value.splice(idx, 1);
};

onMounted(() => {
  invoke("get_data", { path: localStorage.getItem(RECEIVER_LIST) })
    .then((res) => (receiverList.value = JSON.parse(res as string)))
    .catch((e) => ElMessage.error(e));
});

watch(
  receiverList,
  (value) => {
    invoke("update_data", {
      path: localStorage.getItem(RECEIVER_LIST),
      data: JSON.stringify(value),
    }).catch((e) => ElMessage.error(e));
  },
  { deep: true }
);
</script>

<style scoped>
.el-container {
  display: flex;
  flex-direction: column;
  gap: 20px;
}
header {
  display: flex;
  justify-content: flex-end;
}
</style>
