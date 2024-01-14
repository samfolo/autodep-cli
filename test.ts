import nodeStdLibImport from 'fs';
import nodeModuleImport from 'some-node-module';

import aliasedImport from '@alias/aliasedModule';

import localFileImport from '../localFile';

import relativeImport from './relativeModule';


const add = (a: number, b: number) => a + b;

export const someExport = 42;

export default add;