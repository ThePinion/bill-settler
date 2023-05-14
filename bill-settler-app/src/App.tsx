import type { Component } from 'solid-js';

import logo from './logo.svg';
import styles from './App.module.scss';
import { A, Route, Routes } from '@solidjs/router';
import BeforeLayout from './components/before/Layout';
import Login from './components/before/Login';
import Home from './components/home/Home';
import Expenses from './components/expenses/Expenses';
import Groups from './components/groups/Groups';
import NewExpense from './components/new_expense/NewExpense';
import Group from './components/groups/Group';
import NewGroup from './components/new_group/NewGroup';
import DetailedExpense from './components/detailed_expense/DetailedExpense';

const App: Component = () => {

  return (
    <div>
      <Routes>
        <Route
          path="/before"
          component={BeforeLayout}
        >
          <Route path="/login" component={Login} />
        </Route>
        {/* <Route path="/profile" component={Profile} /> */}
        <Route path="/home" component={Home} />
        <Route path="/expenses" component={Expenses}/>
        <Route path="/groups" component={Groups}/>
        <Route path="/new-expense" component={NewExpense} />
        <Route path="/group/:id" component={Group}/>
        <Route path="/new-group" component={NewGroup} />
        <Route path="/expense/:id" component={DetailedExpense}/>
      </Routes>
    </div>
  );
};

export default App;
