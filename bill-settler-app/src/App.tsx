import type { Component } from 'solid-js';

import logo from './logo.svg';
import styles from './App.module.scss';
import { A, Route, Routes } from '@solidjs/router';
import BeforeLayout from './components/before/Layout';
import Login from './components/before/Login';

const App: Component = () => {
  return (
    <>
      <Routes>
        <Route
          path="/before"
          component={BeforeLayout}
        >
          <Route path="/login" component={Login} />
        </Route>
      </Routes>
    </>
  );
};

export default App;
