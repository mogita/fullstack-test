module.exports = {
  root: true,
  extends: ['eslint:recommended'],
  rules: {
    // Allow unused variables that start with underscore
    '@typescript-eslint/no-unused-vars': [
      'error',
      {
        argsIgnorePattern: '^_',
        varsIgnorePattern: '^_',
        caughtErrorsIgnorePattern: '^_',
      },
    ],
  },
}
