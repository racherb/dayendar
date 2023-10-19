# Generating a GPG key and configuring Git for Signed Commit

Install GPG. On Linux and Mac it generally comes preinstalled. On Windows you can install it through Gpg4win.

Generate your GPG key:

```bash
gpg --full-generate-key
```

Follow the prompts. It's recommended to use an email associated with your GitHub/GitLab account and a secure password.

Verify that the key has been generated:

```bash
gpg --list-secret-keys --keyid-format LONG
```

This will list the private keys generated. Copy the ID of the key.

Export the public key:

```bash
gpg --armor --export YOUR_KEY_ID
```

This will generate your public key in ASCII format.

Add the public key to GitHub/GitLab by pasting the result in the GPG keys section.

Configure Git to use this key:

```bash
git config --global user.signingkey YOUR_KEY_ID
git config --global commit.gpgsign true
```

Sign commits by adding -S when you git commit.
With this Git will be configured to sign all commits with your generated GPG key.
