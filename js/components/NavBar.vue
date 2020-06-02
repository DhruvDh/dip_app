<template>
  <b-navbar shadow>
    <template slot="start">
      <b-navbar-item
        tag="router-link"
        :to="{ path: '/ass_1' }"
      >
        Assignment 1
      </b-navbar-item>

      <b-navbar-item
        tag="router-link"
        :to="{ path: '/ass_2' }"
      >
        Assignment 2
      </b-navbar-item>

       <b-navbar-item
        tag="router-link"
        :to="{ path: '/ass_3' }"
      >
        Assignment 3
      </b-navbar-item>

      <b-navbar-item
        tag="router-link"
        :to="{ path: '/viewer' }"
      >
        Viewer
      </b-navbar-item>
    </template>

    <template slot="end">
      <b-navbar-item
        href="https://unccdip.zulipchat.com/"
        target="_blank"
      >
        <img src="img/zulip_logo.png">
      </b-navbar-item>
    </template>
  </b-navbar>
</template>
