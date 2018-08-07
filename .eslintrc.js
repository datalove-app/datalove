module.exports = {
  env: {
    browser: true,
  },

  parser: 'typescript-eslint-parser',
  parserOptions: {},
  extends: [
    'eslint:recommended',
    'plugin:react/recommended',
    'airbnb',
  ],
  plugins: [
    'babel',
    'typescript',
    'jsx-a11y',
    'import',
  ],

  rules: {
    // file
    'eol-last': 'error',
    'indent': ['error', 2],
    'max-len': ['warn', {
      code: 80,
      tabWidth: 2,
      ignoreRegExpLiterals: true,
      ignoreTemplateLiterals: true,
      ignoreTrailingComments: true,
      ignoreUrls: true,
    }],
    'no-multiple-empty-lines': ['error', {
      max: 2,
      maxEOF: 1,
    }],
    'no-tabs': 'error',

    // main
    'comma-dangle': ['error', 'always-multiline'],
    'consistent-return': 'error',
    'func-call-spacing': 'error',
    'func-names': ['error', 'always'],
    'function-paren-newline': ['error', 'consistent'],
    'new-cap': ['error', {
      capIsNew: false,
    }],
    'no-nested-ternary': 'off',
    'no-plusplus': 'error',
    'no-restricted-global': 'warn',
    'no-undef': 'off',  // TODO: see https://github.com/eslint/typescript-eslint-parser/issues/77
    'no-undefined': 'error',
    'no-underscore-dangle': ['warn', {
      allowAfterSuper: true,
      allowAfterThis: true,
      enforceInMethodNames: true,
    }],
    'no-unused-vars': ['error', {
      argsIgnorePattern: '^_',
      ignoreRestSiblings: true,
      varsIgnorePattern: '^_',
    }],
    'no-use-before-define': ['warn', {
      functions: false,
    }],
    'no-void': 'off',
    'object-curly-newline': 'off',
    'prefer-arrow-callback': 'off',
    'quotes': ['error', 'single', {
      avoidEscape: true,
    }],

    // import
    'import/extensions': 'off',
    'import/no-extraneous-dependencies': 'off',
    'import/no-unresolved': 'off',

    // typescript
    'typescript/adjacent-overload-signatures': 'error',
    'typescript/class-name-casing': 'error',
    // TODO: 'typescript/explicit-function-return-type': 'error',
    'typescript/explicit-member-accessibility': 'error',
    'typescript/interface-name-prefix': ['error', 'always'],
    'typescript/member-delimiter-style': ['error', {
      delimiter: 'comma',
      requireLast: true,
      ignoreSingleLine: false,
    }],
    'typescript/member-naming': ['error', {
      private: '^_',
    }],
    'typescript/member-ordering': ['error', {
      default: [
        'static-field',
        'static-method',
        'instance-field',
        'constructor',
        'instance-method',
      ],
    }],
    'typescript/no-angle-bracket-type-assertion': 'error',
    'typescript/no-array-constructor': 'error',
    'typescript/no-empty-interface': 'error',
    'typescript/no-explicit-any': 'warn',
    'typescript/no-triple-slash-reference': 'error',
    'typescript/no-use-before-define': 'error',

    // react/jsx
    'react/boolean-prop-naming': ['error', {
      rule: '^(is|has)[A-Z]([A-Za-z0-9]?)+',
      message: 'It is better if your prop `{{ propName }}`` matches this pattern: /{{ pattern }}/',
    }],
    'react/button-has-type': 'error',
    'react/destructuring-assignment': ['error', 'always'],
    'react/no-array-index-key': 'error',
    'react/no-danger': 'error',
    'react/no-did-mount-set-state': 'error',
    'react/no-did-update-set-state': 'error',
    'react/no-multi-comp': ['error', {
      ignoreStateless: true,
    }],
    'react/no-this-in-sfc': 'error',
    'react/no-typos': 'error',
    'react/prefer-stateless-function': ['error', {
      ignorePureComponents: true,
    }],
    'react/sort-comp': ['error', {
      order: [
        'type-annotations',
        'static-methods',
        'lifecycle',
        'everything-else',
        'render',
      ],
    }],
    'react/sort-prop-types': ['error', {
      callbacksLast: true,
      ignoreCase: true,
      requiredFirst: true,
    }],
    'react/style-prop-object': 'error',

    'react/jsx-equals-spacing': ['error', 'never'],
    'react/jsx-filename-extension': ['error', {
      extensions: [ '.jsx', '.tsx' ],
    }],
    'react/jsx-no-bind': 'error',
    'react/jsx-indent': ['error', 2],
    'react/jsx-indent-props': ['error', 2],
    'react/jsx-pascal-case': ['error'],
    'react/jsx-tag-spacing': ['error', {
      beforeClosing: 'never',
    }],
  },
};
