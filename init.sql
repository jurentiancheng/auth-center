-- =====================================================
-- Auth Center 数据库初始化脚本
-- 数据库: MySQL
-- 生成时间: 基于 SeaORM Entity 模型
-- =====================================================

-- 创建数据库
CREATE DATABASE IF NOT EXISTS `auth_center` DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

USE `auth_center`;

-- =====================================================
-- 基础表
-- =====================================================

-- 用户表
DROP TABLE IF EXISTS `user`;
CREATE TABLE `user` (
    `id` BIGINT NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `user_name` VARCHAR(255) NOT NULL COMMENT '用户名',
    `real_name` VARCHAR(255) DEFAULT NULL COMMENT '真实姓名',
    `password` VARCHAR(255) DEFAULT NULL COMMENT '密码',
    `status` TINYINT DEFAULT 0 COMMENT '状态',
    `type` TINYINT DEFAULT 0 COMMENT '类型',
    `email` VARCHAR(255) DEFAULT NULL COMMENT '邮箱',
    `area_code` VARCHAR(20) DEFAULT NULL COMMENT '区号',
    `phone` VARCHAR(50) DEFAULT NULL COMMENT '手机号',
    `remark` VARCHAR(500) DEFAULT NULL COMMENT '备注',
    `is_del` TINYINT DEFAULT 0 COMMENT '是否删除: 0-否, 1-是',
    `head_pic` VARCHAR(500) DEFAULT NULL COMMENT '头像',
    `create_time` DATETIME DEFAULT NULL COMMENT '创建时间',
    `update_time` DATETIME DEFAULT NULL COMMENT '更新时间',
    `create_by` BIGINT DEFAULT 0 COMMENT '创建人',
    `update_by` BIGINT DEFAULT 0 COMMENT '更新人',
    `is_admin` TINYINT DEFAULT 0 COMMENT '是否管理员: 0-否, 1-是',
    `open_id` VARCHAR(255) DEFAULT NULL COMMENT 'OpenID',
    `last_login_time` DATETIME DEFAULT NULL COMMENT '最后登录时间',
    `wechat_open_id` VARCHAR(255) DEFAULT NULL COMMENT '微信OpenID',
    `wechat_union_id` VARCHAR(255) DEFAULT NULL COMMENT '微信UnionID',
    PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='用户表';

-- 用户信息表
DROP TABLE IF EXISTS `user_info`;
CREATE TABLE `user_info` (
    `id` BIGINT NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `user_code` VARCHAR(100) DEFAULT NULL COMMENT '用户编码',
    `username` VARCHAR(255) DEFAULT NULL COMMENT '用户名',
    `login_status` VARCHAR(50) DEFAULT NULL COMMENT '登录状态',
    `english_name` VARCHAR(255) DEFAULT NULL COMMENT '英文名',
    `real_name` VARCHAR(255) NOT NULL COMMENT '真实姓名',
    `nick_name` VARCHAR(255) DEFAULT NULL COMMENT '昵称',
    `cellphone` VARCHAR(50) DEFAULT NULL COMMENT '手机号',
    `gender` VARCHAR(10) DEFAULT NULL COMMENT '性别',
    `portrait` VARCHAR(500) DEFAULT NULL COMMENT '头像',
    `user_type` VARCHAR(50) NOT NULL COMMENT '用户类型',
    `birthday` DATE DEFAULT NULL COMMENT '生日',
    `password` VARCHAR(255) DEFAULT NULL COMMENT '密码',
    `id_card_type` VARCHAR(50) DEFAULT NULL COMMENT '证件类型',
    `id_card_no` VARCHAR(100) DEFAULT NULL COMMENT '证件号码',
    `email` VARCHAR(255) DEFAULT NULL COMMENT '邮箱',
    `qq` VARCHAR(50) DEFAULT NULL COMMENT 'QQ',
    `wx_union_id` VARCHAR(255) DEFAULT NULL COMMENT '微信UnionID',
    `wx_open_id` VARCHAR(255) DEFAULT NULL COMMENT '微信OpenID',
    `wx_mini_open_id` VARCHAR(255) DEFAULT NULL COMMENT '微信小程序OpenID',
    `address` VARCHAR(500) DEFAULT NULL COMMENT '地址',
    `parent_code` VARCHAR(100) DEFAULT NULL COMMENT '父级编码',
    `path` VARCHAR(500) DEFAULT NULL COMMENT '路径',
    `effective_date` DATE DEFAULT NULL COMMENT '生效日期',
    `invalid_date` DATE DEFAULT NULL COMMENT '失效日期',
    `status` VARCHAR(50) DEFAULT NULL COMMENT '状态',
    `org_code` VARCHAR(100) DEFAULT NULL COMMENT '组织编码',
    `department_code` VARCHAR(100) DEFAULT NULL COMMENT '部门编码',
    `position_code` VARCHAR(100) DEFAULT NULL COMMENT '岗位编码',
    `extra` JSON DEFAULT NULL COMMENT '扩展信息',
    `remark` VARCHAR(500) DEFAULT NULL COMMENT '备注',
    `is_del` TINYINT DEFAULT 0 COMMENT '是否删除: 0-否, 1-是',
    `create_time` DATETIME DEFAULT NULL COMMENT '创建时间',
    `update_time` DATETIME DEFAULT NULL COMMENT '更新时间',
    `create_by` BIGINT DEFAULT 0 COMMENT '创建人',
    `update_by` BIGINT DEFAULT 0 COMMENT '更新人',
    `rec_sign` VARCHAR(255) DEFAULT NULL COMMENT '记录签名',
    PRIMARY KEY (`id`),
    INDEX `idx_user_code` (`user_code`),
    INDEX `idx_org_code` (`org_code`),
    INDEX `idx_department_code` (`department_code`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='用户信息表';

-- 用户微信信息表
DROP TABLE IF EXISTS `user_wechat_info`;
CREATE TABLE `user_wechat_info` (
    `id` BIGINT NOT NULL COMMENT '主键ID',
    `union_id` VARCHAR(255) DEFAULT NULL COMMENT '微信UnionID',
    `wechat_open_id` VARCHAR(255) DEFAULT NULL COMMENT '微信公众号OpenID',
    `mini_open_id` VARCHAR(255) DEFAULT NULL COMMENT '微信小程序OpenID',
    `nickname` VARCHAR(255) DEFAULT NULL COMMENT '昵称',
    `language` VARCHAR(50) DEFAULT NULL COMMENT '语言',
    `subscribe` TINYINT DEFAULT NULL COMMENT '是否关注',
    `head_img_url` VARCHAR(500) DEFAULT NULL COMMENT '头像URL',
    `subscribe_time` BIGINT DEFAULT NULL COMMENT '关注时间',
    `remark` VARCHAR(500) DEFAULT NULL COMMENT '备注',
    `group_id` INT DEFAULT NULL COMMENT '分组ID',
    `tag_ids` JSON DEFAULT NULL COMMENT '标签ID列表',
    `subscribe_scene` VARCHAR(100) DEFAULT NULL COMMENT '关注场景',
    `qr_scene` VARCHAR(100) DEFAULT NULL COMMENT '二维码场景',
    `qr_scene_str` VARCHAR(255) DEFAULT NULL COMMENT '二维码场景字符串',
    `app_id` VARCHAR(100) DEFAULT NULL COMMENT '应用ID',
    `app_type` VARCHAR(50) DEFAULT NULL COMMENT '应用类型',
    `is_del` TINYINT DEFAULT 0 COMMENT '是否删除: 0-否, 1-是',
    `create_time` DATETIME DEFAULT NULL COMMENT '创建时间',
    `update_time` DATETIME DEFAULT NULL COMMENT '更新时间',
    `create_by` BIGINT DEFAULT 0 COMMENT '创建人',
    `update_by` BIGINT DEFAULT 0 COMMENT '更新人',
    PRIMARY KEY (`id`),
    UNIQUE INDEX `uk_union_id` (`union_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='用户微信信息表';

-- 组织表
DROP TABLE IF EXISTS `organization`;
CREATE TABLE `organization` (
    `id` BIGINT NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `code` VARCHAR(100) DEFAULT NULL COMMENT '组织编码',
    `parent_code` VARCHAR(100) DEFAULT NULL COMMENT '父级编码',
    `name` VARCHAR(255) DEFAULT NULL COMMENT '组织名称',
    `type` VARCHAR(50) DEFAULT NULL COMMENT '组织类型',
    `contacts` VARCHAR(255) DEFAULT NULL COMMENT '联系人',
    `cellphone` VARCHAR(50) DEFAULT NULL COMMENT '联系电话',
    `email` VARCHAR(255) DEFAULT NULL COMMENT '邮箱',
    `uscc` VARCHAR(100) DEFAULT NULL COMMENT '统一社会信用代码',
    `business_license` VARCHAR(500) DEFAULT NULL COMMENT '营业执照',
    `id_card_front` VARCHAR(500) DEFAULT NULL COMMENT '身份证正面',
    `id_card_back` VARCHAR(500) DEFAULT NULL COMMENT '身份证背面',
    `invalid_date` DATE DEFAULT NULL COMMENT '失效日期',
    `status` VARCHAR(50) DEFAULT NULL COMMENT '状态',
    `extra` JSON DEFAULT NULL COMMENT '扩展信息',
    `remark` VARCHAR(500) DEFAULT NULL COMMENT '备注',
    `path` VARCHAR(500) DEFAULT NULL COMMENT '路径',
    `is_del` TINYINT DEFAULT 0 COMMENT '是否删除: 0-否, 1-是',
    `create_time` DATETIME DEFAULT NULL COMMENT '创建时间',
    `update_time` DATETIME DEFAULT NULL COMMENT '更新时间',
    `create_by` BIGINT DEFAULT 0 COMMENT '创建人',
    `update_by` BIGINT DEFAULT 0 COMMENT '更新人',
    `rec_sign` VARCHAR(255) DEFAULT NULL COMMENT '记录签名',
    PRIMARY KEY (`id`),
    INDEX `idx_code` (`code`),
    INDEX `idx_parent_code` (`parent_code`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='组织表';

-- 部门表
DROP TABLE IF EXISTS `department`;
CREATE TABLE `department` (
    `id` BIGINT NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `code` VARCHAR(100) DEFAULT NULL COMMENT '部门编码',
    `parent_code` VARCHAR(100) DEFAULT NULL COMMENT '父级编码',
    `name` VARCHAR(255) NOT NULL COMMENT '部门名称',
    `description` VARCHAR(500) DEFAULT NULL COMMENT '描述',
    `org_code` VARCHAR(100) NOT NULL COMMENT '组织编码',
    `extra` JSON DEFAULT NULL COMMENT '扩展信息',
    `remark` VARCHAR(500) DEFAULT NULL COMMENT '备注',
    `path` VARCHAR(500) DEFAULT NULL COMMENT '路径',
    `is_del` TINYINT DEFAULT 0 COMMENT '是否删除: 0-否, 1-是',
    `create_time` DATETIME DEFAULT NULL COMMENT '创建时间',
    `update_time` DATETIME DEFAULT NULL COMMENT '更新时间',
    `create_by` BIGINT DEFAULT 0 COMMENT '创建人',
    `update_by` BIGINT DEFAULT 0 COMMENT '更新人',
    `rec_sign` VARCHAR(255) DEFAULT NULL COMMENT '记录签名',
    PRIMARY KEY (`id`),
    INDEX `idx_code` (`code`),
    INDEX `idx_org_code` (`org_code`),
    INDEX `idx_parent_code` (`parent_code`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='部门表';

-- 岗位表
DROP TABLE IF EXISTS `position`;
CREATE TABLE `position` (
    `id` BIGINT NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `code` VARCHAR(100) DEFAULT NULL COMMENT '岗位编码',
    `parent_code` VARCHAR(100) DEFAULT NULL COMMENT '父级编码',
    `name` VARCHAR(255) NOT NULL COMMENT '岗位名称',
    `description` VARCHAR(500) DEFAULT NULL COMMENT '描述',
    `department_code` VARCHAR(100) DEFAULT NULL COMMENT '部门编码',
    `org_code` VARCHAR(100) DEFAULT NULL COMMENT '组织编码',
    `extra` JSON DEFAULT NULL COMMENT '扩展信息',
    `remark` VARCHAR(500) DEFAULT NULL COMMENT '备注',
    `path` VARCHAR(500) DEFAULT NULL COMMENT '路径',
    `is_del` TINYINT DEFAULT 0 COMMENT '是否删除: 0-否, 1-是',
    `create_time` DATETIME DEFAULT NULL COMMENT '创建时间',
    `update_time` DATETIME DEFAULT NULL COMMENT '更新时间',
    `create_by` BIGINT DEFAULT 0 COMMENT '创建人',
    `update_by` BIGINT DEFAULT 0 COMMENT '更新人',
    PRIMARY KEY (`id`),
    INDEX `idx_code` (`code`),
    INDEX `idx_org_code` (`org_code`),
    INDEX `idx_department_code` (`department_code`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='岗位表';

-- 用户组表
DROP TABLE IF EXISTS `group`;
CREATE TABLE `group` (
    `id` BIGINT NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `code` VARCHAR(100) DEFAULT NULL COMMENT '用户组编码',
    `parent_code` VARCHAR(100) DEFAULT NULL COMMENT '父级编码',
    `name` VARCHAR(255) NOT NULL COMMENT '用户组名称',
    `type` VARCHAR(50) NOT NULL COMMENT '用户组类型',
    `description` VARCHAR(500) DEFAULT NULL COMMENT '描述',
    `application` VARCHAR(100) NOT NULL COMMENT '应用',
    `org_code` VARCHAR(100) DEFAULT NULL COMMENT '组织编码',
    `extra` JSON DEFAULT NULL COMMENT '扩展信息',
    `remark` VARCHAR(500) DEFAULT NULL COMMENT '备注',
    `path` VARCHAR(500) DEFAULT NULL COMMENT '路径',
    `is_del` TINYINT DEFAULT 0 COMMENT '是否删除: 0-否, 1-是',
    `create_time` DATETIME DEFAULT NULL COMMENT '创建时间',
    `update_time` DATETIME DEFAULT NULL COMMENT '更新时间',
    `create_by` BIGINT DEFAULT 0 COMMENT '创建人',
    `update_by` BIGINT DEFAULT 0 COMMENT '更新人',
    PRIMARY KEY (`id`),
    INDEX `idx_code` (`code`),
    INDEX `idx_org_code` (`org_code`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='用户组表';

-- 角色表
DROP TABLE IF EXISTS `role`;
CREATE TABLE `role` (
    `id` BIGINT NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `code` VARCHAR(100) DEFAULT NULL COMMENT '角色编码',
    `parent_code` VARCHAR(100) DEFAULT NULL COMMENT '父级编码',
    `name` VARCHAR(255) NOT NULL COMMENT '角色名称',
    `description` VARCHAR(500) DEFAULT NULL COMMENT '描述',
    `application` VARCHAR(100) DEFAULT NULL COMMENT '应用',
    `permissions` JSON DEFAULT NULL COMMENT '权限列表',
    `org_code` VARCHAR(100) DEFAULT NULL COMMENT '组织编码',
    `extra` JSON DEFAULT NULL COMMENT '扩展信息',
    `remark` VARCHAR(500) DEFAULT NULL COMMENT '备注',
    `role_type` TINYINT NOT NULL COMMENT '角色类型',
    `path` VARCHAR(500) DEFAULT NULL COMMENT '路径',
    `is_del` TINYINT DEFAULT 0 COMMENT '是否删除: 0-否, 1-是',
    `create_time` DATETIME DEFAULT NULL COMMENT '创建时间',
    `update_time` DATETIME DEFAULT NULL COMMENT '更新时间',
    `create_by` BIGINT DEFAULT 0 COMMENT '创建人',
    `update_by` BIGINT DEFAULT 0 COMMENT '更新人',
    `rec_sign` VARCHAR(255) DEFAULT NULL COMMENT '记录签名',
    PRIMARY KEY (`id`),
    INDEX `idx_code` (`code`),
    INDEX `idx_org_code` (`org_code`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='角色表';

-- 权限表
DROP TABLE IF EXISTS `permission`;
CREATE TABLE `permission` (
    `id` BIGINT NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `uuid` VARCHAR(100) NOT NULL COMMENT 'UUID',
    `parent_uuid` VARCHAR(100) DEFAULT NULL COMMENT '父级UUID',
    `code` VARCHAR(100) DEFAULT NULL COMMENT '权限编码',
    `name` VARCHAR(255) NOT NULL COMMENT '权限名称',
    `description` VARCHAR(500) DEFAULT NULL COMMENT '描述',
    `application` VARCHAR(100) DEFAULT NULL COMMENT '应用',
    `sort` INT DEFAULT NULL COMMENT '排序',
    `node_type` VARCHAR(50) DEFAULT NULL COMMENT '节点类型',
    `link_url` VARCHAR(500) DEFAULT NULL COMMENT '链接URL',
    `ico_url` VARCHAR(500) DEFAULT NULL COMMENT '图标URL',
    `level` INT DEFAULT NULL COMMENT '层级',
    `path` VARCHAR(500) DEFAULT NULL COMMENT '路径',
    `org_code` VARCHAR(100) DEFAULT NULL COMMENT '组织编码',
    `extra` JSON DEFAULT NULL COMMENT '扩展信息',
    `remark` VARCHAR(500) DEFAULT NULL COMMENT '备注',
    `is_del` TINYINT DEFAULT 0 COMMENT '是否删除: 0-否, 1-是',
    `create_time` DATETIME DEFAULT NULL COMMENT '创建时间',
    `update_time` DATETIME DEFAULT NULL COMMENT '更新时间',
    `create_by` BIGINT DEFAULT 0 COMMENT '创建人',
    `update_by` BIGINT DEFAULT 0 COMMENT '更新人',
    PRIMARY KEY (`id`),
    INDEX `idx_uuid` (`uuid`),
    INDEX `idx_code` (`code`),
    INDEX `idx_org_code` (`org_code`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='权限表';

-- 系统配置表
DROP TABLE IF EXISTS `system_config`;
CREATE TABLE `system_config` (
    `id` BIGINT NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `config_key` VARCHAR(255) DEFAULT NULL COMMENT '配置键',
    `config_value` TEXT DEFAULT NULL COMMENT '配置值',
    `config_type` VARCHAR(50) DEFAULT NULL COMMENT '配置类型',
    `description` VARCHAR(500) DEFAULT NULL COMMENT '描述',
    `status` VARCHAR(50) NOT NULL COMMENT '状态',
    `org_code` VARCHAR(100) DEFAULT NULL COMMENT '组织编码',
    `is_del` TINYINT DEFAULT 0 COMMENT '是否删除: 0-否, 1-是',
    `create_time` DATETIME DEFAULT NULL COMMENT '创建时间',
    `update_time` DATETIME DEFAULT NULL COMMENT '更新时间',
    `create_by` BIGINT DEFAULT 0 COMMENT '创建人',
    `update_by` BIGINT DEFAULT 0 COMMENT '更新人',
    PRIMARY KEY (`id`),
    INDEX `idx_config_key` (`config_key`),
    INDEX `idx_org_code` (`org_code`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='系统配置表';

-- =====================================================
-- 关联表
-- =====================================================

-- 用户-角色关联表
DROP TABLE IF EXISTS `user_role_ref`;
CREATE TABLE `user_role_ref` (
    `id` BIGINT NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `user_code` VARCHAR(100) NOT NULL COMMENT '用户编码',
    `role_code` VARCHAR(100) NOT NULL COMMENT '角色编码',
    `org_code` VARCHAR(100) DEFAULT NULL COMMENT '组织编码',
    `is_del` TINYINT DEFAULT 0 COMMENT '是否删除: 0-否, 1-是',
    `create_time` DATETIME DEFAULT NULL COMMENT '创建时间',
    `update_time` DATETIME DEFAULT NULL COMMENT '更新时间',
    `create_by` BIGINT DEFAULT 0 COMMENT '创建人',
    `update_by` BIGINT DEFAULT 0 COMMENT '更新人',
    `rec_sign` VARCHAR(255) DEFAULT NULL COMMENT '记录签名',
    PRIMARY KEY (`id`),
    INDEX `idx_user_code` (`user_code`),
    INDEX `idx_role_code` (`role_code`),
    INDEX `idx_org_code` (`org_code`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='用户-角色关联表';

-- 用户-用户组关联表
DROP TABLE IF EXISTS `user_group_ref`;
CREATE TABLE `user_group_ref` (
    `id` BIGINT NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `user_code` VARCHAR(100) NOT NULL COMMENT '用户编码',
    `group_code` VARCHAR(100) NOT NULL COMMENT '用户组编码',
    `org_code` VARCHAR(100) DEFAULT NULL COMMENT '组织编码',
    `is_del` TINYINT DEFAULT 0 COMMENT '是否删除: 0-否, 1-是',
    `create_time` DATETIME DEFAULT NULL COMMENT '创建时间',
    `update_time` DATETIME DEFAULT NULL COMMENT '更新时间',
    `create_by` BIGINT DEFAULT 0 COMMENT '创建人',
    `update_by` BIGINT DEFAULT 0 COMMENT '更新人',
    PRIMARY KEY (`id`),
    INDEX `idx_user_code` (`user_code`),
    INDEX `idx_group_code` (`group_code`),
    INDEX `idx_org_code` (`org_code`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='用户-用户组关联表';

-- 用户组-角色关联表
DROP TABLE IF EXISTS `group_role_ref`;
CREATE TABLE `group_role_ref` (
    `id` BIGINT NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `group_code` VARCHAR(100) NOT NULL COMMENT '用户组编码',
    `role_code` VARCHAR(100) NOT NULL COMMENT '角色编码',
    `org_code` VARCHAR(100) DEFAULT NULL COMMENT '组织编码',
    `is_del` TINYINT DEFAULT 0 COMMENT '是否删除: 0-否, 1-是',
    `create_time` DATETIME DEFAULT NULL COMMENT '创建时间',
    `update_time` DATETIME DEFAULT NULL COMMENT '更新时间',
    `create_by` BIGINT DEFAULT 0 COMMENT '创建人',
    `update_by` BIGINT DEFAULT 0 COMMENT '更新人',
    PRIMARY KEY (`id`),
    INDEX `idx_group_code` (`group_code`),
    INDEX `idx_role_code` (`role_code`),
    INDEX `idx_org_code` (`org_code`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='用户组-角色关联表';

-- 部门-角色关联表
DROP TABLE IF EXISTS `department_role_ref`;
CREATE TABLE `department_role_ref` (
    `id` BIGINT NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `department_code` VARCHAR(100) NOT NULL COMMENT '部门编码',
    `role_code` VARCHAR(100) NOT NULL COMMENT '角色编码',
    `org_code` VARCHAR(100) DEFAULT NULL COMMENT '组织编码',
    `is_del` TINYINT DEFAULT 0 COMMENT '是否删除: 0-否, 1-是',
    `create_time` DATETIME DEFAULT NULL COMMENT '创建时间',
    `update_time` DATETIME DEFAULT NULL COMMENT '更新时间',
    `create_by` BIGINT DEFAULT 0 COMMENT '创建人',
    `update_by` BIGINT DEFAULT 0 COMMENT '更新人',
    PRIMARY KEY (`id`),
    INDEX `idx_department_code` (`department_code`),
    INDEX `idx_role_code` (`role_code`),
    INDEX `idx_org_code` (`org_code`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='部门-角色关联表';

-- 岗位-角色关联表
DROP TABLE IF EXISTS `position_role_ref`;
CREATE TABLE `position_role_ref` (
    `id` BIGINT NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `position_code` VARCHAR(100) NOT NULL COMMENT '岗位编码',
    `role_code` VARCHAR(100) NOT NULL COMMENT '角色编码',
    `org_code` VARCHAR(100) DEFAULT NULL COMMENT '组织编码',
    `is_del` TINYINT DEFAULT 0 COMMENT '是否删除: 0-否, 1-是',
    `create_time` DATETIME DEFAULT NULL COMMENT '创建时间',
    `update_time` DATETIME DEFAULT NULL COMMENT '更新时间',
    `create_by` BIGINT DEFAULT 0 COMMENT '创建人',
    `update_by` BIGINT DEFAULT 0 COMMENT '更新人',
    PRIMARY KEY (`id`),
    INDEX `idx_position_code` (`position_code`),
    INDEX `idx_role_code` (`role_code`),
    INDEX `idx_org_code` (`org_code`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='岗位-角色关联表';

-- 组织-角色关联表
DROP TABLE IF EXISTS `organization_role_ref`;
CREATE TABLE `organization_role_ref` (
    `id` BIGINT NOT NULL AUTO_INCREMENT COMMENT '主键ID',
    `org_code` VARCHAR(100) NOT NULL COMMENT '组织编码',
    `role_code` VARCHAR(100) NOT NULL COMMENT '角色编码',
    `is_del` TINYINT DEFAULT 0 COMMENT '是否删除: 0-否, 1-是',
    `create_time` DATETIME DEFAULT NULL COMMENT '创建时间',
    `update_time` DATETIME DEFAULT NULL COMMENT '更新时间',
    `create_by` BIGINT DEFAULT 0 COMMENT '创建人',
    `update_by` BIGINT DEFAULT 0 COMMENT '更新人',
    PRIMARY KEY (`id`),
    INDEX `idx_org_code` (`org_code`),
    INDEX `idx_role_code` (`role_code`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='组织-角色关联表';
