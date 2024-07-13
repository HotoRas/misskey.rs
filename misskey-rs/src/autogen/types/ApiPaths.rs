// With power of JSON
pub let paths = object!{
    "/admin/meta": {
    /**
     * admin/meta
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:admin:meta*
     */
    post: operations["admin___meta"],
  },
  "/admin/abuse-user-reports": {
    /**
     * admin/abuse-user-reports
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:admin:abuse-user-reports*
     */
    post: operations["admin___abuse-user-reports"],
  },
  "/admin/abuse-report/notification-recipient/list": {
    /**
     * admin/abuse-report/notification-recipient/list
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes* / **Permission**: *read:admin:abuse-report:notification-recipient*
     */
    post: operations["admin___abuse-report___notification-recipient___list"],
  },
  "/admin/abuse-report/notification-recipient/show": {
    /**
     * admin/abuse-report/notification-recipient/show
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes* / **Permission**: *read:admin:abuse-report:notification-recipient*
     */
    post: operations["admin___abuse-report___notification-recipient___show"],
  },
  "/admin/abuse-report/notification-recipient/create": {
    /**
     * admin/abuse-report/notification-recipient/create
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes* / **Permission**: *write:admin:abuse-report:notification-recipient*
     */
    post: operations["admin___abuse-report___notification-recipient___create"],
  },
  "/admin/abuse-report/notification-recipient/update": {
    /**
     * admin/abuse-report/notification-recipient/update
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes* / **Permission**: *write:admin:abuse-report:notification-recipient*
     */
    post: operations["admin___abuse-report___notification-recipient___update"],
  },
  "/admin/abuse-report/notification-recipient/delete": {
    /**
     * admin/abuse-report/notification-recipient/delete
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes* / **Permission**: *write:admin:abuse-report:notification-recipient*
     */
    post: operations["admin___abuse-report___notification-recipient___delete"],
  },
  "/admin/accounts/create": {
    /**
     * admin/accounts/create
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["admin___accounts___create"],
  },
  "/admin/accounts/delete": {
    /**
     * admin/accounts/delete
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:account*
     */
    post: operations["admin___accounts___delete"],
  },
  "/admin/accounts/find-by-email": {
    /**
     * admin/accounts/find-by-email
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:admin:account*
     */
    post: operations["admin___accounts___find-by-email"],
  },
  "/admin/ad/create": {
    /**
     * admin/ad/create
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:ad*
     */
    post: operations["admin___ad___create"],
  },
  "/admin/ad/delete": {
    /**
     * admin/ad/delete
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:ad*
     */
    post: operations["admin___ad___delete"],
  },
  "/admin/ad/list": {
    /**
     * admin/ad/list
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:admin:ad*
     */
    post: operations["admin___ad___list"],
  },
  "/admin/ad/update": {
    /**
     * admin/ad/update
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:ad*
     */
    post: operations["admin___ad___update"],
  },
  "/admin/announcements/create": {
    /**
     * admin/announcements/create
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:announcements*
     */
    post: operations["admin___announcements___create"],
  },
  "/admin/announcements/delete": {
    /**
     * admin/announcements/delete
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:announcements*
     */
    post: operations["admin___announcements___delete"],
  },
  "/admin/announcements/list": {
    /**
     * admin/announcements/list
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:admin:announcements*
     */
    post: operations["admin___announcements___list"],
  },
  "/admin/announcements/update": {
    /**
     * admin/announcements/update
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:announcements*
     */
    post: operations["admin___announcements___update"],
  },
  "/admin/avatar-decorations/create": {
    /**
     * admin/avatar-decorations/create
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:avatar-decorations*
     */
    post: operations["admin___avatar-decorations___create"],
  },
  "/admin/avatar-decorations/delete": {
    /**
     * admin/avatar-decorations/delete
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:avatar-decorations*
     */
    post: operations["admin___avatar-decorations___delete"],
  },
  "/admin/avatar-decorations/list": {
    /**
     * admin/avatar-decorations/list
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:admin:avatar-decorations*
     */
    post: operations["admin___avatar-decorations___list"],
  },
  "/admin/avatar-decorations/update": {
    /**
     * admin/avatar-decorations/update
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:avatar-decorations*
     */
    post: operations["admin___avatar-decorations___update"],
  },
  "/admin/delete-all-files-of-a-user": {
    /**
     * admin/delete-all-files-of-a-user
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:delete-all-files-of-a-user*
     */
    post: operations["admin___delete-all-files-of-a-user"],
  },
  "/admin/unset-user-avatar": {
    /**
     * admin/unset-user-avatar
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:unset-user-avatar*
     */
    post: operations["admin___unset-user-avatar"],
  },
  "/admin/unset-user-banner": {
    /**
     * admin/unset-user-banner
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:unset-user-banner*
     */
    post: operations["admin___unset-user-banner"],
  },
  "/admin/drive/clean-remote-files": {
    /**
     * admin/drive/clean-remote-files
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:drive*
     */
    post: operations["admin___drive___clean-remote-files"],
  },
  "/admin/drive/cleanup": {
    /**
     * admin/drive/cleanup
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:drive*
     */
    post: operations["admin___drive___cleanup"],
  },
  "/admin/drive/files": {
    /**
     * admin/drive/files
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:admin:drive*
     */
    post: operations["admin___drive___files"],
  },
  "/admin/drive/show-file": {
    /**
     * admin/drive/show-file
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:admin:drive*
     */
    post: operations["admin___drive___show-file"],
  },
  "/admin/emoji/add-aliases-bulk": {
    /**
     * admin/emoji/add-aliases-bulk
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:emoji*
     */
    post: operations["admin___emoji___add-aliases-bulk"],
  },
  "/admin/emoji/add": {
    /**
     * admin/emoji/add
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:emoji*
     */
    post: operations["admin___emoji___add"],
  },
  "/admin/emoji/copy": {
    /**
     * admin/emoji/copy
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:emoji*
     */
    post: operations["admin___emoji___copy"],
  },
  "/admin/emoji/delete-bulk": {
    /**
     * admin/emoji/delete-bulk
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:emoji*
     */
    post: operations["admin___emoji___delete-bulk"],
  },
  "/admin/emoji/delete": {
    /**
     * admin/emoji/delete
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:emoji*
     */
    post: operations["admin___emoji___delete"],
  },
  "/admin/emoji/import-zip": {
    /**
     * admin/emoji/import-zip
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    post: operations["admin___emoji___import-zip"],
  },
  "/admin/emoji/list-remote": {
    /**
     * admin/emoji/list-remote
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:admin:emoji*
     */
    post: operations["admin___emoji___list-remote"],
  },
  "/admin/emoji/list": {
    /**
     * admin/emoji/list
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:admin:emoji*
     */
    post: operations["admin___emoji___list"],
  },
  "/admin/emoji/remove-aliases-bulk": {
    /**
     * admin/emoji/remove-aliases-bulk
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:emoji*
     */
    post: operations["admin___emoji___remove-aliases-bulk"],
  },
  "/admin/emoji/set-aliases-bulk": {
    /**
     * admin/emoji/set-aliases-bulk
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:emoji*
     */
    post: operations["admin___emoji___set-aliases-bulk"],
  },
  "/admin/emoji/set-category-bulk": {
    /**
     * admin/emoji/set-category-bulk
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:emoji*
     */
    post: operations["admin___emoji___set-category-bulk"],
  },
  "/admin/emoji/set-license-bulk": {
    /**
     * admin/emoji/set-license-bulk
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:emoji*
     */
    post: operations["admin___emoji___set-license-bulk"],
  },
  "/admin/emoji/update": {
    /**
     * admin/emoji/update
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:emoji*
     */
    post: operations["admin___emoji___update"],
  },
  "/admin/federation/delete-all-files": {
    /**
     * admin/federation/delete-all-files
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:federation*
     */
    post: operations["admin___federation___delete-all-files"],
  },
  "/admin/federation/refresh-remote-instance-metadata": {
    /**
     * admin/federation/refresh-remote-instance-metadata
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:federation*
     */
    post: operations["admin___federation___refresh-remote-instance-metadata"],
  },
  "/admin/federation/remove-all-following": {
    /**
     * admin/federation/remove-all-following
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:federation*
     */
    post: operations["admin___federation___remove-all-following"],
  },
  "/admin/federation/update-instance": {
    /**
     * admin/federation/update-instance
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:federation*
     */
    post: operations["admin___federation___update-instance"],
  },
  "/admin/get-index-stats": {
    /**
     * admin/get-index-stats
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:admin:index-stats*
     */
    post: operations["admin___get-index-stats"],
  },
  "/admin/get-table-stats": {
    /**
     * admin/get-table-stats
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:admin:table-stats*
     */
    post: operations["admin___get-table-stats"],
  },
  "/admin/get-user-ips": {
    /**
     * admin/get-user-ips
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:admin:user-ips*
     */
    post: operations["admin___get-user-ips"],
  },
  "/admin/invite/create": {
    /**
     * admin/invite/create
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:invite-codes*
     */
    post: operations["admin___invite___create"],
  },
  "/admin/invite/list": {
    /**
     * admin/invite/list
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:admin:invite-codes*
     */
    post: operations["admin___invite___list"],
  },
  "/admin/promo/create": {
    /**
     * admin/promo/create
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:promo*
     */
    post: operations["admin___promo___create"],
  },
  "/admin/queue/clear": {
    /**
     * admin/queue/clear
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:queue*
     */
    post: operations["admin___queue___clear"],
  },
  "/admin/queue/deliver-delayed": {
    /**
     * admin/queue/deliver-delayed
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:admin:queue*
     */
    post: operations["admin___queue___deliver-delayed"],
  },
  "/admin/queue/inbox-delayed": {
    /**
     * admin/queue/inbox-delayed
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:admin:queue*
     */
    post: operations["admin___queue___inbox-delayed"],
  },
  "/admin/queue/promote": {
    /**
     * admin/queue/promote
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:queue*
     */
    post: operations["admin___queue___promote"],
  },
  "/admin/queue/stats": {
    /**
     * admin/queue/stats
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:admin:emoji*
     */
    post: operations["admin___queue___stats"],
  },
  "/admin/relays/add": {
    /**
     * admin/relays/add
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:relays*
     */
    post: operations["admin___relays___add"],
  },
  "/admin/relays/list": {
    /**
     * admin/relays/list
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:admin:relays*
     */
    post: operations["admin___relays___list"],
  },
  "/admin/relays/remove": {
    /**
     * admin/relays/remove
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:relays*
     */
    post: operations["admin___relays___remove"],
  },
  "/admin/reset-password": {
    /**
     * admin/reset-password
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:reset-password*
     */
    post: operations["admin___reset-password"],
  },
  "/admin/resolve-abuse-user-report": {
    /**
     * admin/resolve-abuse-user-report
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:resolve-abuse-user-report*
     */
    post: operations["admin___resolve-abuse-user-report"],
  },
  "/admin/send-email": {
    /**
     * admin/send-email
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:send-email*
     */
    post: operations["admin___send-email"],
  },
  "/admin/server-info": {
    /**
     * admin/server-info
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:admin:server-info*
     */
    post: operations["admin___server-info"],
  },
  "/admin/show-moderation-logs": {
    /**
     * admin/show-moderation-logs
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:admin:show-moderation-log*
     */
    post: operations["admin___show-moderation-logs"],
  },
  "/admin/show-user": {
    /**
     * admin/show-user
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:admin:show-user*
     */
    post: operations["admin___show-user"],
  },
  "/admin/show-users": {
    /**
     * admin/show-users
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:admin:show-user*
     */
    post: operations["admin___show-users"],
  },
  "/admin/suspend-user": {
    /**
     * admin/suspend-user
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:suspend-user*
     */
    post: operations["admin___suspend-user"],
  },
  "/admin/unsuspend-user": {
    /**
     * admin/unsuspend-user
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:unsuspend-user*
     */
    post: operations["admin___unsuspend-user"],
  },
  "/admin/update-meta": {
    /**
     * admin/update-meta
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:meta*
     */
    post: operations["admin___update-meta"],
  },
  "/admin/delete-account": {
    /**
     * admin/delete-account
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:delete-account*
     */
    post: operations["admin___delete-account"],
  },
  "/admin/update-user-note": {
    /**
     * admin/update-user-note
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:user-note*
     */
    post: operations["admin___update-user-note"],
  },
  "/admin/roles/create": {
    /**
     * admin/roles/create
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:roles*
     */
    post: operations["admin___roles___create"],
  },
  "/admin/roles/delete": {
    /**
     * admin/roles/delete
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:roles*
     */
    post: operations["admin___roles___delete"],
  },
  "/admin/roles/list": {
    /**
     * admin/roles/list
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:admin:roles*
     */
    post: operations["admin___roles___list"],
  },
  "/admin/roles/show": {
    /**
     * admin/roles/show
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:admin:roles*
     */
    post: operations["admin___roles___show"],
  },
  "/admin/roles/update": {
    /**
     * admin/roles/update
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:roles*
     */
    post: operations["admin___roles___update"],
  },
  "/admin/roles/assign": {
    /**
     * admin/roles/assign
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:roles*
     */
    post: operations["admin___roles___assign"],
  },
  "/admin/roles/unassign": {
    /**
     * admin/roles/unassign
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:roles*
     */
    post: operations["admin___roles___unassign"],
  },
  "/admin/roles/update-default-policies": {
    /**
     * admin/roles/update-default-policies
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:roles*
     */
    post: operations["admin___roles___update-default-policies"],
  },
  "/admin/roles/users": {
    /**
     * admin/roles/users
     * @description No description provided.
     *
     * **Credential required**: *No* / **Permission**: *read:admin:roles*
     */
    post: operations["admin___roles___users"],
  },
  "/admin/system-webhook/create": {
    /**
     * admin/system-webhook/create
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes* / **Permission**: *write:admin:system-webhook*
     */
    post: operations["admin___system-webhook___create"],
  },
  "/admin/system-webhook/delete": {
    /**
     * admin/system-webhook/delete
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes* / **Permission**: *write:admin:system-webhook*
     */
    post: operations["admin___system-webhook___delete"],
  },
  "/admin/system-webhook/list": {
    /**
     * admin/system-webhook/list
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes* / **Permission**: *write:admin:system-webhook*
     */
    post: operations["admin___system-webhook___list"],
  },
  "/admin/system-webhook/show": {
    /**
     * admin/system-webhook/show
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes* / **Permission**: *write:admin:system-webhook*
     */
    post: operations["admin___system-webhook___show"],
  },
  "/admin/system-webhook/update": {
    /**
     * admin/system-webhook/update
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes* / **Permission**: *write:admin:system-webhook*
     */
    post: operations["admin___system-webhook___update"],
  },
  "/announcements": {
    /**
     * announcements
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["announcements"],
  },
  "/announcements/show": {
    /**
     * announcements/show
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["announcements___show"],
  },
  "/antennas/create": {
    /**
     * antennas/create
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    post: operations["antennas___create"],
  },
  "/antennas/delete": {
    /**
     * antennas/delete
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    post: operations["antennas___delete"],
  },
  "/antennas/list": {
    /**
     * antennas/list
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    post: operations["antennas___list"],
  },
  "/antennas/notes": {
    /**
     * antennas/notes
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    post: operations["antennas___notes"],
  },
  "/antennas/show": {
    /**
     * antennas/show
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    post: operations["antennas___show"],
  },
  "/antennas/update": {
    /**
     * antennas/update
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    post: operations["antennas___update"],
  },
  "/ap/get": {
    /**
     * ap/get
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:federation*
     */
    post: operations["ap___get"],
  },
  "/ap/show": {
    /**
     * ap/show
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    post: operations["ap___show"],
  },
  "/app/create": {
    /**
     * app/create
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["app___create"],
  },
  "/app/show": {
    /**
     * app/show
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["app___show"],
  },
  "/auth/accept": {
    /**
     * auth/accept
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    post: operations["auth___accept"],
  },
  "/auth/session/generate": {
    /**
     * auth/session/generate
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["auth___session___generate"],
  },
  "/auth/session/show": {
    /**
     * auth/session/show
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["auth___session___show"],
  },
  "/auth/session/userkey": {
    /**
     * auth/session/userkey
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["auth___session___userkey"],
  },
  "/blocking/create": {
    /**
     * blocking/create
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:blocks*
     */
    post: operations["blocking___create"],
  },
  "/blocking/delete": {
    /**
     * blocking/delete
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:blocks*
     */
    post: operations["blocking___delete"],
  },
  "/blocking/list": {
    /**
     * blocking/list
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:blocks*
     */
    post: operations["blocking___list"],
  },
  "/channels/create": {
    /**
     * channels/create
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:channels*
     */
    post: operations["channels___create"],
  },
  "/channels/featured": {
    /**
     * channels/featured
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["channels___featured"],
  },
  "/channels/follow": {
    /**
     * channels/follow
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:channels*
     */
    post: operations["channels___follow"],
  },
  "/channels/followed": {
    /**
     * channels/followed
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:channels*
     */
    post: operations["channels___followed"],
  },
  "/channels/owned": {
    /**
     * channels/owned
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:channels*
     */
    post: operations["channels___owned"],
  },
  "/channels/show": {
    /**
     * channels/show
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["channels___show"],
  },
  "/channels/timeline": {
    /**
     * channels/timeline
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["channels___timeline"],
  },
  "/channels/unfollow": {
    /**
     * channels/unfollow
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:channels*
     */
    post: operations["channels___unfollow"],
  },
  "/channels/update": {
    /**
     * channels/update
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:channels*
     */
    post: operations["channels___update"],
  },
  "/channels/favorite": {
    /**
     * channels/favorite
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:channels*
     */
    post: operations["channels___favorite"],
  },
  "/channels/unfavorite": {
    /**
     * channels/unfavorite
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:channels*
     */
    post: operations["channels___unfavorite"],
  },
  "/channels/my-favorites": {
    /**
     * channels/my-favorites
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:channels*
     */
    post: operations["channels___my-favorites"],
  },
  "/channels/search": {
    /**
     * channels/search
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["channels___search"],
  },
  "/charts/active-users": {
    /**
     * charts/active-users
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    get: operations["charts___active-users"],
    /**
     * charts/active-users
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["charts___active-users"],
  },
  "/charts/ap-request": {
    /**
     * charts/ap-request
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    get: operations["charts___ap-request"],
    /**
     * charts/ap-request
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["charts___ap-request"],
  },
  "/charts/drive": {
    /**
     * charts/drive
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    get: operations["charts___drive"],
    /**
     * charts/drive
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["charts___drive"],
  },
  "/charts/federation": {
    /**
     * charts/federation
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    get: operations["charts___federation"],
    /**
     * charts/federation
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["charts___federation"],
  },
  "/charts/instance": {
    /**
     * charts/instance
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    get: operations["charts___instance"],
    /**
     * charts/instance
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["charts___instance"],
  },
  "/charts/notes": {
    /**
     * charts/notes
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    get: operations["charts___notes"],
    /**
     * charts/notes
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["charts___notes"],
  },
  "/charts/user/drive": {
    /**
     * charts/user/drive
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    get: operations["charts___user___drive"],
    /**
     * charts/user/drive
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["charts___user___drive"],
  },
  "/charts/user/following": {
    /**
     * charts/user/following
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    get: operations["charts___user___following"],
    /**
     * charts/user/following
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["charts___user___following"],
  },
  "/charts/user/notes": {
    /**
     * charts/user/notes
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    get: operations["charts___user___notes"],
    /**
     * charts/user/notes
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["charts___user___notes"],
  },
  "/charts/user/pv": {
    /**
     * charts/user/pv
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    get: operations["charts___user___pv"],
    /**
     * charts/user/pv
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["charts___user___pv"],
  },
  "/charts/user/reactions": {
    /**
     * charts/user/reactions
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    get: operations["charts___user___reactions"],
    /**
     * charts/user/reactions
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["charts___user___reactions"],
  },
  "/charts/users": {
    /**
     * charts/users
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    get: operations["charts___users"],
    /**
     * charts/users
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["charts___users"],
  },
  "/clips/add-note": {
    /**
     * clips/add-note
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    post: operations["clips___add-note"],
  },
  "/clips/remove-note": {
    /**
     * clips/remove-note
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    post: operations["clips___remove-note"],
  },
  "/clips/create": {
    /**
     * clips/create
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    post: operations["clips___create"],
  },
  "/clips/delete": {
    /**
     * clips/delete
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    post: operations["clips___delete"],
  },
  "/clips/list": {
    /**
     * clips/list
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    post: operations["clips___list"],
  },
  "/clips/notes": {
    /**
     * clips/notes
     * @description No description provided.
     *
     * **Credential required**: *No* / **Permission**: *read:account*
     */
    post: operations["clips___notes"],
  },
  "/clips/show": {
    /**
     * clips/show
     * @description No description provided.
     *
     * **Credential required**: *No* / **Permission**: *read:account*
     */
    post: operations["clips___show"],
  },
  "/clips/update": {
    /**
     * clips/update
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    post: operations["clips___update"],
  },
  "/clips/favorite": {
    /**
     * clips/favorite
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:clip-favorite*
     */
    post: operations["clips___favorite"],
  },
  "/clips/unfavorite": {
    /**
     * clips/unfavorite
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:clip-favorite*
     */
    post: operations["clips___unfavorite"],
  },
  "/clips/my-favorites": {
    /**
     * clips/my-favorites
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:clip-favorite*
     */
    post: operations["clips___my-favorites"],
  },
  "/drive": {
    /**
     * drive
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:drive*
     */
    post: operations["drive"],
  },
  "/drive/files": {
    /**
     * drive/files
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:drive*
     */
    post: operations["drive___files"],
  },
  "/drive/files/attached-notes": {
    /**
     * drive/files/attached-notes
     * @description Find the notes to which the given file is attached.
     *
     * **Credential required**: *Yes* / **Permission**: *read:drive*
     */
    post: operations["drive___files___attached-notes"],
  },
  "/drive/files/check-existence": {
    /**
     * drive/files/check-existence
     * @description Check if a given file exists.
     *
     * **Credential required**: *Yes* / **Permission**: *read:drive*
     */
    post: operations["drive___files___check-existence"],
  },
  "/drive/files/create": {
    /**
     * drive/files/create
     * @description Upload a new drive file.
     *
     * **Credential required**: *Yes* / **Permission**: *write:drive*
     */
    post: operations["drive___files___create"],
  },
  "/drive/files/delete": {
    /**
     * drive/files/delete
     * @description Delete an existing drive file.
     *
     * **Credential required**: *Yes* / **Permission**: *write:drive*
     */
    post: operations["drive___files___delete"],
  },
  "/drive/files/find-by-hash": {
    /**
     * drive/files/find-by-hash
     * @description Search for a drive file by a hash of the contents.
     *
     * **Credential required**: *Yes* / **Permission**: *read:drive*
     */
    post: operations["drive___files___find-by-hash"],
  },
  "/drive/files/find": {
    /**
     * drive/files/find
     * @description Search for a drive file by the given parameters.
     *
     * **Credential required**: *Yes* / **Permission**: *read:drive*
     */
    post: operations["drive___files___find"],
  },
  "/drive/files/show": {
    /**
     * drive/files/show
     * @description Show the properties of a drive file.
     *
     * **Credential required**: *Yes* / **Permission**: *read:drive*
     */
    post: operations["drive___files___show"],
  },
  "/drive/files/update": {
    /**
     * drive/files/update
     * @description Update the properties of a drive file.
     *
     * **Credential required**: *Yes* / **Permission**: *write:drive*
     */
    post: operations["drive___files___update"],
  },
  "/drive/files/upload-from-url": {
    /**
     * drive/files/upload-from-url
     * @description Request the server to download a new drive file from the specified URL.
     *
     * **Credential required**: *Yes* / **Permission**: *write:drive*
     */
    post: operations["drive___files___upload-from-url"],
  },
  "/drive/folders": {
    /**
     * drive/folders
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:drive*
     */
    post: operations["drive___folders"],
  },
  "/drive/folders/create": {
    /**
     * drive/folders/create
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:drive*
     */
    post: operations["drive___folders___create"],
  },
  "/drive/folders/delete": {
    /**
     * drive/folders/delete
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:drive*
     */
    post: operations["drive___folders___delete"],
  },
  "/drive/folders/find": {
    /**
     * drive/folders/find
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:drive*
     */
    post: operations["drive___folders___find"],
  },
  "/drive/folders/show": {
    /**
     * drive/folders/show
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:drive*
     */
    post: operations["drive___folders___show"],
  },
  "/drive/folders/update": {
    /**
     * drive/folders/update
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:drive*
     */
    post: operations["drive___folders___update"],
  },
  "/drive/stream": {
    /**
     * drive/stream
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:drive*
     */
    post: operations["drive___stream"],
  },
  "/email-address/available": {
    /**
     * email-address/available
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["email-address___available"],
  },
  "/endpoint": {
    /**
     * endpoint
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["endpoint"],
  },
  "/endpoints": {
    /**
     * endpoints
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["endpoints"],
  },
  "/export-custom-emojis": {
    /**
     * export-custom-emojis
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    post: operations["export-custom-emojis"],
  },
  "/federation/followers": {
    /**
     * federation/followers
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["federation___followers"],
  },
  "/federation/following": {
    /**
     * federation/following
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["federation___following"],
  },
  "/federation/instances": {
    /**
     * federation/instances
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    get: operations["federation___instances"],
    /**
     * federation/instances
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["federation___instances"],
  },
  "/federation/show-instance": {
    /**
     * federation/show-instance
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["federation___show-instance"],
  },
  "/federation/update-remote-user": {
    /**
     * federation/update-remote-user
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["federation___update-remote-user"],
  },
  "/federation/users": {
    /**
     * federation/users
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["federation___users"],
  },
  "/federation/stats": {
    /**
     * federation/stats
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    get: operations["federation___stats"],
    /**
     * federation/stats
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["federation___stats"],
  },
  "/following/create": {
    /**
     * following/create
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:following*
     */
    post: operations["following___create"],
  },
  "/following/delete": {
    /**
     * following/delete
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:following*
     */
    post: operations["following___delete"],
  },
  "/following/update": {
    /**
     * following/update
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:following*
     */
    post: operations["following___update"],
  },
  "/following/update-all": {
    /**
     * following/update-all
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:following*
     */
    post: operations["following___update-all"],
  },
  "/following/invalidate": {
    /**
     * following/invalidate
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:following*
     */
    post: operations["following___invalidate"],
  },
  "/following/requests/accept": {
    /**
     * following/requests/accept
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:following*
     */
    post: operations["following___requests___accept"],
  },
  "/following/requests/cancel": {
    /**
     * following/requests/cancel
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:following*
     */
    post: operations["following___requests___cancel"],
  },
  "/following/requests/list": {
    /**
     * following/requests/list
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:following*
     */
    post: operations["following___requests___list"],
  },
  "/following/requests/reject": {
    /**
     * following/requests/reject
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:following*
     */
    post: operations["following___requests___reject"],
  },
  "/gallery/featured": {
    /**
     * gallery/featured
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["gallery___featured"],
  },
  "/gallery/popular": {
    /**
     * gallery/popular
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["gallery___popular"],
  },
  "/gallery/posts": {
    /**
     * gallery/posts
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["gallery___posts"],
  },
  "/gallery/posts/create": {
    /**
     * gallery/posts/create
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:gallery*
     */
    post: operations["gallery___posts___create"],
  },
  "/gallery/posts/delete": {
    /**
     * gallery/posts/delete
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:gallery*
     */
    post: operations["gallery___posts___delete"],
  },
  "/gallery/posts/like": {
    /**
     * gallery/posts/like
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:gallery-likes*
     */
    post: operations["gallery___posts___like"],
  },
  "/gallery/posts/show": {
    /**
     * gallery/posts/show
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["gallery___posts___show"],
  },
  "/gallery/posts/unlike": {
    /**
     * gallery/posts/unlike
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:gallery-likes*
     */
    post: operations["gallery___posts___unlike"],
  },
  "/gallery/posts/update": {
    /**
     * gallery/posts/update
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:gallery*
     */
    post: operations["gallery___posts___update"],
  },
  "/get-online-users-count": {
    /**
     * get-online-users-count
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    get: operations["get-online-users-count"],
    /**
     * get-online-users-count
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["get-online-users-count"],
  },
  "/get-avatar-decorations": {
    /**
     * get-avatar-decorations
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["get-avatar-decorations"],
  },
  "/hashtags/list": {
    /**
     * hashtags/list
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["hashtags___list"],
  },
  "/hashtags/search": {
    /**
     * hashtags/search
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["hashtags___search"],
  },
  "/hashtags/show": {
    /**
     * hashtags/show
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["hashtags___show"],
  },
  "/hashtags/trend": {
    /**
     * hashtags/trend
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    get: operations["hashtags___trend"],
    /**
     * hashtags/trend
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["hashtags___trend"],
  },
  "/hashtags/users": {
    /**
     * hashtags/users
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["hashtags___users"],
  },
  "/i": {
    /**
     * i
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    post: operations["i"],
  },
  "/i/2fa/done": {
    /**
     * i/2fa/done
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    post: operations["i___2fa___done"],
  },
  "/i/2fa/key-done": {
    /**
     * i/2fa/key-done
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    post: operations["i___2fa___key-done"],
  },
  "/i/2fa/password-less": {
    /**
     * i/2fa/password-less
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    post: operations["i___2fa___password-less"],
  },
  "/i/2fa/register-key": {
    /**
     * i/2fa/register-key
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    post: operations["i___2fa___register-key"],
  },
  "/i/2fa/register": {
    /**
     * i/2fa/register
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    post: operations["i___2fa___register"],
  },
  "/i/2fa/update-key": {
    /**
     * i/2fa/update-key
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    post: operations["i___2fa___update-key"],
  },
  "/i/2fa/remove-key": {
    /**
     * i/2fa/remove-key
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    post: operations["i___2fa___remove-key"],
  },
  "/i/2fa/unregister": {
    /**
     * i/2fa/unregister
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    post: operations["i___2fa___unregister"],
  },
  "/i/apps": {
    /**
     * i/apps
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    post: operations["i___apps"],
  },
  "/i/authorized-apps": {
    /**
     * i/authorized-apps
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    post: operations["i___authorized-apps"],
  },
  "/i/claim-achievement": {
    /**
     * i/claim-achievement
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    post: operations["i___claim-achievement"],
  },
  "/i/change-password": {
    /**
     * i/change-password
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    post: operations["i___change-password"],
  },
  "/i/delete-account": {
    /**
     * i/delete-account
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    post: operations["i___delete-account"],
  },
  "/i/export-blocking": {
    /**
     * i/export-blocking
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    post: operations["i___export-blocking"],
  },
  "/i/export-following": {
    /**
     * i/export-following
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    post: operations["i___export-following"],
  },
  "/i/export-mute": {
    /**
     * i/export-mute
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    post: operations["i___export-mute"],
  },
  "/i/export-notes": {
    /**
     * i/export-notes
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    post: operations["i___export-notes"],
  },
  "/i/export-clips": {
    /**
     * i/export-clips
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    post: operations["i___export-clips"],
  },
  "/i/export-favorites": {
    /**
     * i/export-favorites
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    post: operations["i___export-favorites"],
  },
  "/i/export-user-lists": {
    /**
     * i/export-user-lists
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    post: operations["i___export-user-lists"],
  },
  "/i/export-antennas": {
    /**
     * i/export-antennas
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    post: operations["i___export-antennas"],
  },
  "/i/favorites": {
    /**
     * i/favorites
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:favorites*
     */
    post: operations["i___favorites"],
  },
  "/i/gallery/likes": {
    /**
     * i/gallery/likes
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:gallery-likes*
     */
    post: operations["i___gallery___likes"],
  },
  "/i/gallery/posts": {
    /**
     * i/gallery/posts
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:gallery*
     */
    post: operations["i___gallery___posts"],
  },
  "/i/import-blocking": {
    /**
     * i/import-blocking
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    post: operations["i___import-blocking"],
  },
  "/i/import-following": {
    /**
     * i/import-following
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    post: operations["i___import-following"],
  },
  "/i/import-muting": {
    /**
     * i/import-muting
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    post: operations["i___import-muting"],
  },
  "/i/import-user-lists": {
    /**
     * i/import-user-lists
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    post: operations["i___import-user-lists"],
  },
  "/i/import-antennas": {
    /**
     * i/import-antennas
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    post: operations["i___import-antennas"],
  },
  "/i/notifications": {
    /**
     * i/notifications
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:notifications*
     */
    post: operations["i___notifications"],
  },
  "/i/notifications-grouped": {
    /**
     * i/notifications-grouped
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:notifications*
     */
    post: operations["i___notifications-grouped"],
  },
  "/i/page-likes": {
    /**
     * i/page-likes
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:page-likes*
     */
    post: operations["i___page-likes"],
  },
  "/i/pages": {
    /**
     * i/pages
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:pages*
     */
    post: operations["i___pages"],
  },
  "/i/pin": {
    /**
     * i/pin
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    post: operations["i___pin"],
  },
  "/i/read-all-unread-notes": {
    /**
     * i/read-all-unread-notes
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    post: operations["i___read-all-unread-notes"],
  },
  "/i/read-announcement": {
    /**
     * i/read-announcement
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    post: operations["i___read-announcement"],
  },
  "/i/regenerate-token": {
    /**
     * i/regenerate-token
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    post: operations["i___regenerate-token"],
  },
  "/i/registry/get-all": {
    /**
     * i/registry/get-all
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    post: operations["i___registry___get-all"],
  },
  "/i/registry/get-detail": {
    /**
     * i/registry/get-detail
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    post: operations["i___registry___get-detail"],
  },
  "/i/registry/get": {
    /**
     * i/registry/get
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    post: operations["i___registry___get"],
  },
  "/i/registry/keys-with-type": {
    /**
     * i/registry/keys-with-type
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    post: operations["i___registry___keys-with-type"],
  },
  "/i/registry/keys": {
    /**
     * i/registry/keys
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    post: operations["i___registry___keys"],
  },
  "/i/registry/remove": {
    /**
     * i/registry/remove
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    post: operations["i___registry___remove"],
  },
  "/i/registry/scopes-with-domain": {
    /**
     * i/registry/scopes-with-domain
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    post: operations["i___registry___scopes-with-domain"],
  },
  "/i/registry/set": {
    /**
     * i/registry/set
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    post: operations["i___registry___set"],
  },
  "/i/revoke-token": {
    /**
     * i/revoke-token
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    post: operations["i___revoke-token"],
  },
  "/i/signin-history": {
    /**
     * i/signin-history
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    post: operations["i___signin-history"],
  },
  "/i/unpin": {
    /**
     * i/unpin
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    post: operations["i___unpin"],
  },
  "/i/update-email": {
    /**
     * i/update-email
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    post: operations["i___update-email"],
  },
  "/i/update": {
    /**
     * i/update
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    post: operations["i___update"],
  },
  "/i/move": {
    /**
     * i/move
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    post: operations["i___move"],
  },
  "/i/webhooks/create": {
    /**
     * i/webhooks/create
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    post: operations["i___webhooks___create"],
  },
  "/i/webhooks/list": {
    /**
     * i/webhooks/list
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    post: operations["i___webhooks___list"],
  },
  "/i/webhooks/show": {
    /**
     * i/webhooks/show
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    post: operations["i___webhooks___show"],
  },
  "/i/webhooks/update": {
    /**
     * i/webhooks/update
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    post: operations["i___webhooks___update"],
  },
  "/i/webhooks/delete": {
    /**
     * i/webhooks/delete
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    post: operations["i___webhooks___delete"],
  },
  "/invite/create": {
    /**
     * invite/create
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:invite-codes*
     */
    post: operations["invite___create"],
  },
  "/invite/delete": {
    /**
     * invite/delete
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:invite-codes*
     */
    post: operations["invite___delete"],
  },
  "/invite/list": {
    /**
     * invite/list
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:invite-codes*
     */
    post: operations["invite___list"],
  },
  "/invite/limit": {
    /**
     * invite/limit
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:invite-codes*
     */
    post: operations["invite___limit"],
  },
  "/meta": {
    /**
     * meta
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["meta"],
  },
  "/emojis": {
    /**
     * emojis
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    get: operations["emojis"],
    /**
     * emojis
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["emojis"],
  },
  "/emoji": {
    /**
     * emoji
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    get: operations["emoji"],
    /**
     * emoji
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["emoji"],
  },
  "/miauth/gen-token": {
    /**
     * miauth/gen-token
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    post: operations["miauth___gen-token"],
  },
  "/mute/create": {
    /**
     * mute/create
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:mutes*
     */
    post: operations["mute___create"],
  },
  "/mute/delete": {
    /**
     * mute/delete
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:mutes*
     */
    post: operations["mute___delete"],
  },
  "/mute/list": {
    /**
     * mute/list
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:mutes*
     */
    post: operations["mute___list"],
  },
  "/renote-mute/create": {
    /**
     * renote-mute/create
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:mutes*
     */
    post: operations["renote-mute___create"],
  },
  "/renote-mute/delete": {
    /**
     * renote-mute/delete
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:mutes*
     */
    post: operations["renote-mute___delete"],
  },
  "/renote-mute/list": {
    /**
     * renote-mute/list
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:mutes*
     */
    post: operations["renote-mute___list"],
  },
  "/my/apps": {
    /**
     * my/apps
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    post: operations["my___apps"],
  },
  "/notes": {
    /**
     * notes
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["notes"],
  },
  "/notes/children": {
    /**
     * notes/children
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["notes___children"],
  },
  "/notes/clips": {
    /**
     * notes/clips
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["notes___clips"],
  },
  "/notes/conversation": {
    /**
     * notes/conversation
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["notes___conversation"],
  },
  "/notes/create": {
    /**
     * notes/create
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:notes*
     */
    post: operations["notes___create"],
  },
  "/notes/delete": {
    /**
     * notes/delete
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:notes*
     */
    post: operations["notes___delete"],
  },
  "/notes/favorites/create": {
    /**
     * notes/favorites/create
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:favorites*
     */
    post: operations["notes___favorites___create"],
  },
  "/notes/favorites/delete": {
    /**
     * notes/favorites/delete
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:favorites*
     */
    post: operations["notes___favorites___delete"],
  },
  "/notes/featured": {
    /**
     * notes/featured
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    get: operations["notes___featured"],
    /**
     * notes/featured
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["notes___featured"],
  },
  "/notes/global-timeline": {
    /**
     * notes/global-timeline
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["notes___global-timeline"],
  },
  "/notes/hybrid-timeline": {
    /**
     * notes/hybrid-timeline
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    post: operations["notes___hybrid-timeline"],
  },
  "/notes/local-timeline": {
    /**
     * notes/local-timeline
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["notes___local-timeline"],
  },
  "/notes/mentions": {
    /**
     * notes/mentions
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    post: operations["notes___mentions"],
  },
  "/notes/polls/recommendation": {
    /**
     * notes/polls/recommendation
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    post: operations["notes___polls___recommendation"],
  },
  "/notes/polls/vote": {
    /**
     * notes/polls/vote
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:votes*
     */
    post: operations["notes___polls___vote"],
  },
  "/notes/reactions": {
    /**
     * notes/reactions
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    get: operations["notes___reactions"],
    /**
     * notes/reactions
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["notes___reactions"],
  },
  "/notes/reactions/create": {
    /**
     * notes/reactions/create
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:reactions*
     */
    post: operations["notes___reactions___create"],
  },
  "/notes/reactions/delete": {
    /**
     * notes/reactions/delete
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:reactions*
     */
    post: operations["notes___reactions___delete"],
  },
  "/notes/renotes": {
    /**
     * notes/renotes
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["notes___renotes"],
  },
  "/notes/replies": {
    /**
     * notes/replies
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["notes___replies"],
  },
  "/notes/search-by-tag": {
    /**
     * notes/search-by-tag
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["notes___search-by-tag"],
  },
  "/notes/search": {
    /**
     * notes/search
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["notes___search"],
  },
  "/notes/show": {
    /**
     * notes/show
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["notes___show"],
  },
  "/notes/state": {
    /**
     * notes/state
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    post: operations["notes___state"],
  },
  "/notes/thread-muting/create": {
    /**
     * notes/thread-muting/create
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    post: operations["notes___thread-muting___create"],
  },
  "/notes/thread-muting/delete": {
    /**
     * notes/thread-muting/delete
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    post: operations["notes___thread-muting___delete"],
  },
  "/notes/timeline": {
    /**
     * notes/timeline
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    post: operations["notes___timeline"],
  },
  "/notes/translate": {
    /**
     * notes/translate
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    post: operations["notes___translate"],
  },
  "/notes/unrenote": {
    /**
     * notes/unrenote
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:notes*
     */
    post: operations["notes___unrenote"],
  },
  "/notes/user-list-timeline": {
    /**
     * notes/user-list-timeline
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    post: operations["notes___user-list-timeline"],
  },
  "/notifications/create": {
    /**
     * notifications/create
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:notifications*
     */
    post: operations["notifications___create"],
  },
  "/notifications/flush": {
    /**
     * notifications/flush
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:notifications*
     */
    post: operations["notifications___flush"],
  },
  "/notifications/mark-all-as-read": {
    /**
     * notifications/mark-all-as-read
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:notifications*
     */
    post: operations["notifications___mark-all-as-read"],
  },
  "/notifications/test-notification": {
    /**
     * notifications/test-notification
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:notifications*
     */
    post: operations["notifications___test-notification"],
  },
  "/page-push": {
    /**
     * page-push
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    post: operations["page-push"],
  },
  "/pages/create": {
    /**
     * pages/create
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:pages*
     */
    post: operations["pages___create"],
  },
  "/pages/delete": {
    /**
     * pages/delete
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:pages*
     */
    post: operations["pages___delete"],
  },
  "/pages/featured": {
    /**
     * pages/featured
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["pages___featured"],
  },
  "/pages/like": {
    /**
     * pages/like
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:page-likes*
     */
    post: operations["pages___like"],
  },
  "/pages/show": {
    /**
     * pages/show
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["pages___show"],
  },
  "/pages/unlike": {
    /**
     * pages/unlike
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:page-likes*
     */
    post: operations["pages___unlike"],
  },
  "/pages/update": {
    /**
     * pages/update
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:pages*
     */
    post: operations["pages___update"],
  },
  "/flash/create": {
    /**
     * flash/create
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:flash*
     */
    post: operations["flash___create"],
  },
  "/flash/delete": {
    /**
     * flash/delete
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:flash*
     */
    post: operations["flash___delete"],
  },
  "/flash/featured": {
    /**
     * flash/featured
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["flash___featured"],
  },
  "/flash/like": {
    /**
     * flash/like
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:flash-likes*
     */
    post: operations["flash___like"],
  },
  "/flash/show": {
    /**
     * flash/show
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["flash___show"],
  },
  "/flash/unlike": {
    /**
     * flash/unlike
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:flash-likes*
     */
    post: operations["flash___unlike"],
  },
  "/flash/update": {
    /**
     * flash/update
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:flash*
     */
    post: operations["flash___update"],
  },
  "/flash/my": {
    /**
     * flash/my
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:flash*
     */
    post: operations["flash___my"],
  },
  "/flash/my-likes": {
    /**
     * flash/my-likes
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:flash-likes*
     */
    post: operations["flash___my-likes"],
  },
  "/ping": {
    /**
     * ping
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["ping"],
  },
  "/pinned-users": {
    /**
     * pinned-users
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["pinned-users"],
  },
  "/promo/read": {
    /**
     * promo/read
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    post: operations["promo___read"],
  },
  "/roles/list": {
    /**
     * roles/list
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    post: operations["roles___list"],
  },
  "/roles/show": {
    /**
     * roles/show
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["roles___show"],
  },
  "/roles/users": {
    /**
     * roles/users
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["roles___users"],
  },
  "/roles/notes": {
    /**
     * roles/notes
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    post: operations["roles___notes"],
  },
  "/request-reset-password": {
    /**
     * request-reset-password
     * @description Request a users password to be reset.
     *
     * **Credential required**: *No*
     */
    post: operations["request-reset-password"],
  },
  "/reset-db": {
    /**
     * reset-db
     * @description Only available when running with <code>NODE_ENV=testing</code>. Reset the database and flush Redis.
     *
     * **Credential required**: *No*
     */
    post: operations["reset-db"],
  },
  "/reset-password": {
    /**
     * reset-password
     * @description Complete the password reset that was previously requested.
     *
     * **Credential required**: *No*
     */
    post: operations["reset-password"],
  },
  "/server-info": {
    /**
     * server-info
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    get: operations["server-info"],
    /**
     * server-info
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["server-info"],
  },
  "/stats": {
    /**
     * stats
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["stats"],
  },
  "/sw/show-registration": {
    /**
     * sw/show-registration
     * @description Check push notification registration exists.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    post: operations["sw___show-registration"],
  },
  "/sw/update-registration": {
    /**
     * sw/update-registration
     * @description Update push notification registration.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    post: operations["sw___update-registration"],
  },
  "/sw/register": {
    /**
     * sw/register
     * @description Register to receive push notifications.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    post: operations["sw___register"],
  },
  "/sw/unregister": {
    /**
     * sw/unregister
     * @description Unregister from receiving push notifications.
     *
     * **Credential required**: *No*
     */
    post: operations["sw___unregister"],
  },
  "/test": {
    /**
     * test
     * @description Endpoint for testing input validation.
     *
     * **Credential required**: *No*
     */
    post: operations["test"],
  },
  "/username/available": {
    /**
     * username/available
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["username___available"],
  },
  "/users": {
    /**
     * users
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["users"],
  },
  "/users/clips": {
    /**
     * users/clips
     * @description Show all clips this user owns.
     *
     * **Credential required**: *No*
     */
    post: operations["users___clips"],
  },
  "/users/followers": {
    /**
     * users/followers
     * @description Show everyone that follows this user.
     *
     * **Credential required**: *No*
     */
    post: operations["users___followers"],
  },
  "/users/following": {
    /**
     * users/following
     * @description Show everyone that this user is following.
     *
     * **Credential required**: *No*
     */
    post: operations["users___following"],
  },
  "/users/gallery/posts": {
    /**
     * users/gallery/posts
     * @description Show all gallery posts by the given user.
     *
     * **Credential required**: *No*
     */
    post: operations["users___gallery___posts"],
  },
  "/users/get-frequently-replied-users": {
    /**
     * users/get-frequently-replied-users
     * @description Get a list of other users that the specified user frequently replies to.
     *
     * **Credential required**: *No*
     */
    post: operations["users___get-frequently-replied-users"],
  },
  "/users/featured-notes": {
    /**
     * users/featured-notes
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    get: operations["users___featured-notes"],
    /**
     * users/featured-notes
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["users___featured-notes"],
  },
  "/users/lists/create": {
    /**
     * users/lists/create
     * @description Create a new list of users.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    post: operations["users___lists___create"],
  },
  "/users/lists/delete": {
    /**
     * users/lists/delete
     * @description Delete an existing list of users.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    post: operations["users___lists___delete"],
  },
  "/users/lists/list": {
    /**
     * users/lists/list
     * @description Show all lists that the authenticated user has created.
     *
     * **Credential required**: *No* / **Permission**: *read:account*
     */
    post: operations["users___lists___list"],
  },
  "/users/lists/pull": {
    /**
     * users/lists/pull
     * @description Remove a user from a list.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    post: operations["users___lists___pull"],
  },
  "/users/lists/push": {
    /**
     * users/lists/push
     * @description Add a user to an existing list.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    post: operations["users___lists___push"],
  },
  "/users/lists/show": {
    /**
     * users/lists/show
     * @description Show the properties of a list.
     *
     * **Credential required**: *No* / **Permission**: *read:account*
     */
    post: operations["users___lists___show"],
  },
  "/users/lists/favorite": {
    /**
     * users/lists/favorite
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    post: operations["users___lists___favorite"],
  },
  "/users/lists/unfavorite": {
    /**
     * users/lists/unfavorite
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    post: operations["users___lists___unfavorite"],
  },
  "/users/lists/update": {
    /**
     * users/lists/update
     * @description Update the properties of a list.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    post: operations["users___lists___update"],
  },
  "/users/lists/create-from-public": {
    /**
     * users/lists/create-from-public
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    post: operations["users___lists___create-from-public"],
  },
  "/users/lists/update-membership": {
    /**
     * users/lists/update-membership
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    post: operations["users___lists___update-membership"],
  },
  "/users/lists/get-memberships": {
    /**
     * users/lists/get-memberships
     * @description No description provided.
     *
     * **Credential required**: *No* / **Permission**: *read:account*
     */
    post: operations["users___lists___get-memberships"],
  },
  "/users/notes": {
    /**
     * users/notes
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["users___notes"],
  },
  "/users/pages": {
    /**
     * users/pages
     * @description Show all pages this user created.
     *
     * **Credential required**: *No*
     */
    post: operations["users___pages"],
  },
  "/users/flashs": {
    /**
     * users/flashs
     * @description Show all flashs this user created.
     *
     * **Credential required**: *No*
     */
    post: operations["users___flashs"],
  },
  "/users/reactions": {
    /**
     * users/reactions
     * @description Show all reactions this user made.
     *
     * **Credential required**: *No*
     */
    post: operations["users___reactions"],
  },
  "/users/recommendation": {
    /**
     * users/recommendation
     * @description Show users that the authenticated user might be interested to follow.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    post: operations["users___recommendation"],
  },
  "/users/relation": {
    /**
     * users/relation
     * @description Show the different kinds of relations between the authenticated user and the specified user(s).
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    post: operations["users___relation"],
  },
  "/users/report-abuse": {
    /**
     * users/report-abuse
     * @description File a report.
     *
     * **Credential required**: *Yes* / **Permission**: *write:report-abuse*
     */
    post: operations["users___report-abuse"],
  },
  "/users/search-by-username-and-host": {
    /**
     * users/search-by-username-and-host
     * @description Search for a user by username and/or host.
     *
     * **Credential required**: *No*
     */
    post: operations["users___search-by-username-and-host"],
  },
  "/users/search": {
    /**
     * users/search
     * @description Search for users.
     *
     * **Credential required**: *No*
     */
    post: operations["users___search"],
  },
  "/users/show": {
    /**
     * users/show
     * @description Show the properties of a user.
     *
     * **Credential required**: *No*
     */
    post: operations["users___show"],
  },
  "/users/achievements": {
    /**
     * users/achievements
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["users___achievements"],
  },
  "/users/update-memo": {
    /**
     * users/update-memo
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    post: operations["users___update-memo"],
  },
  "/fetch-rss": {
    /**
     * fetch-rss
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    get: operations["fetch-rss"],
    /**
     * fetch-rss
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["fetch-rss"],
  },
  "/fetch-external-resources": {
    /**
     * fetch-external-resources
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    post: operations["fetch-external-resources"],
  },
  "/retention": {
    /**
     * retention
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    get: operations["retention"],
    /**
     * retention
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["retention"],
  },
  "/bubble-game/register": {
    /**
     * bubble-game/register
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    post: operations["bubble-game___register"],
  },
  "/bubble-game/ranking": {
    /**
     * bubble-game/ranking
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    get: operations["bubble-game___ranking"],
    /**
     * bubble-game/ranking
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["bubble-game___ranking"],
  },
  "/reversi/cancel-match": {
    /**
     * reversi/cancel-match
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    post: operations["reversi___cancel-match"],
  },
  "/reversi/games": {
    /**
     * reversi/games
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["reversi___games"],
  },
  "/reversi/match": {
    /**
     * reversi/match
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    post: operations["reversi___match"],
  },
  "/reversi/invitations": {
    /**
     * reversi/invitations
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    post: operations["reversi___invitations"],
  },
  "/reversi/show-game": {
    /**
     * reversi/show-game
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["reversi___show-game"],
  },
  "/reversi/surrender": {
    /**
     * reversi/surrender
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    post: operations["reversi___surrender"],
  },
  "/reversi/verify": {
    /**
     * reversi/verify
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    post: operations["reversi___verify"],
  },
};

pub let operations = object!{

    /**
     * admin/meta
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:admin:meta*
     */
    admin___meta: {
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              cacheRemoteFiles: boolean,
              cacheRemoteSensitiveFiles: boolean,
              emailRequiredForSignup: boolean,
              enableHcaptcha: boolean,
              hcaptchaSiteKey: string | null,
              enableMcaptcha: boolean,
              mcaptchaSiteKey: string | null,
              mcaptchaInstanceUrl: string | null,
              enableRecaptcha: boolean,
              recaptchaSiteKey: string | null,
              enableTurnstile: boolean,
              turnstileSiteKey: string | null,
              swPublickey: string | null,
              /** @default /assets/ai.png */
              mascotImageUrl: string | null,
              bannerUrl: string | null,
              serverErrorImageUrl: string | null,
              infoImageUrl: string | null,
              notFoundImageUrl: string | null,
              iconUrl: string | null,
              app192IconUrl: string | null,
              app512IconUrl: string | null,
              enableEmail: boolean,
              enableServiceWorker: boolean,
              translatorAvailable: boolean,
              silencedHosts?: string[],
              pinnedUsers: string[],
              hiddenTags: string[],
              blockedHosts: string[],
              sensitiveWords: string[],
              prohibitedWords: string[],
              bannedEmailDomains?: string[],
              preservedUsernames: string[],
              hcaptchaSecretKey: string | null,
              mcaptchaSecretKey: string | null,
              recaptchaSecretKey: string | null,
              turnstileSecretKey: string | null,
              sensitiveMediaDetection: string,
              sensitiveMediaDetectionSensitivity: string,
              setSensitiveFlagAutomatically: boolean,
              enableSensitiveMediaDetectionForVideos: boolean,
              /** Format: id */
              proxyAccountId: string | null,
              email: string | null,
              smtpSecure: boolean,
              smtpHost: string | null,
              smtpPort: number | null,
              smtpUser: string | null,
              smtpPass: string | null,
              swPrivateKey: string | null,
              useObjectStorage: boolean,
              objectStorageBaseUrl: string | null,
              objectStorageBucket: string | null,
              objectStoragePrefix: string | null,
              objectStorageEndpoint: string | null,
              objectStorageRegion: string | null,
              objectStoragePort: number | null,
              objectStorageAccessKey: string | null,
              objectStorageSecretKey: string | null,
              objectStorageUseSSL: boolean,
              objectStorageUseProxy: boolean,
              objectStorageSetPublicRead: boolean,
              enableIpLogging: boolean,
              enableActiveEmailValidation: boolean,
              enableVerifymailApi: boolean,
              verifymailAuthKey: string | null,
              enableTruemailApi: boolean,
              truemailInstance: string | null,
              truemailAuthKey: string | null,
              enableChartsForRemoteUser: boolean,
              enableChartsForFederatedInstances: boolean,
              enableServerMachineStats: boolean,
              enableIdenticonGeneration: boolean,
              manifestJsonOverride: string,
              policies: Record<string, never>,
              enableFanoutTimeline: boolean,
              enableFanoutTimelineDbFallback: boolean,
              perLocalUserUserTimelineCacheMax: number,
              perRemoteUserUserTimelineCacheMax: number,
              perUserHomeTimelineCacheMax: number,
              perUserListTimelineCacheMax: number,
              notesPerOneAd: number,
              backgroundImageUrl: string | null,
              deeplAuthKey: string | null,
              deeplIsPro: boolean,
              defaultDarkTheme: string | null,
              defaultLightTheme: string | null,
              description: string | null,
              disableRegistration: boolean,
              impressumUrl: string | null,
              maintainerEmail: string | null,
              maintainerName: string | null,
              name: string | null,
              shortName: string | null,
              objectStorageS3ForcePathStyle: boolean,
              privacyPolicyUrl: string | null,
              inquiryUrl: string | null,
              repositoryUrl: string | null,
              /**
               * @deprecated
               * @description [Deprecated] Use "urlPreviewSummaryProxyUrl" instead.
               */
              summalyProxy: string | null,
              themeColor: string | null,
              tosUrl: string | null,
              uri: string,
              version: string,
              urlPreviewEnabled: boolean,
              urlPreviewTimeout: number,
              urlPreviewMaximumContentLength: number,
              urlPreviewRequireContentLength: boolean,
              urlPreviewUserAgent: string | null,
              urlPreviewSummaryProxyUrl: string | null,
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/abuse-user-reports
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:admin:abuse-user-reports*
     */
    "admin___abuse-user-reports": {
      requestBody: {
        content: {
          "application/json": {
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
            /** @default null */
            state?: string | null,
            /**
             * @default combined
             * @enum {string}
             */
            reporterOrigin?: "combined" | "local" | "remote",
            /**
             * @default combined
             * @enum {string}
             */
            targetUserOrigin?: "combined" | "local" | "remote",
            /** @default false */
            forwarded?: boolean,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": ({
                /**
                 * Format: id
                 * @example xxxxxxxxxx
                 */
                id: string,
                /** Format: date-time */
                createdAt: string,
                comment: string,
                /** @example false */
                resolved: boolean,
                /** Format: id */
                reporterId: string,
                /** Format: id */
                targetUserId: string,
                /** Format: id */
                assigneeId: string | null,
                reporter: components["schemas"]["UserDetailedNotMe"],
                targetUser: components["schemas"]["UserDetailedNotMe"],
                assignee?: components["schemas"]["UserDetailedNotMe"] | null,
              })[],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/abuse-report/notification-recipient/list
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes* / **Permission**: *read:admin:abuse-report:notification-recipient*
     */
    "admin___abuse-report___notification-recipient___list": {
      requestBody: {
        content: {
          "application/json": {
            method?: ("email" | "webhook")[],
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["AbuseReportNotificationRecipient"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/abuse-report/notification-recipient/show
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes* / **Permission**: *read:admin:abuse-report:notification-recipient*
     */
    "admin___abuse-report___notification-recipient___show": {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            id: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["AbuseReportNotificationRecipient"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/abuse-report/notification-recipient/create
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes* / **Permission**: *write:admin:abuse-report:notification-recipient*
     */
    "admin___abuse-report___notification-recipient___create": {
      requestBody: {
        content: {
          "application/json": {
            isActive: boolean,
            name: string,
            /** @enum {string} */
            method: "email" | "webhook",
            /** Format: misskey:id */
            userId?: string,
            /** Format: misskey:id */
            systemWebhookId?: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["AbuseReportNotificationRecipient"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/abuse-report/notification-recipient/update
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes* / **Permission**: *write:admin:abuse-report:notification-recipient*
     */
    "admin___abuse-report___notification-recipient___update": {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            id: string,
            isActive: boolean,
            name: string,
            /** @enum {string} */
            method: "email" | "webhook",
            /** Format: misskey:id */
            userId?: string,
            /** Format: misskey:id */
            systemWebhookId?: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["AbuseReportNotificationRecipient"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/abuse-report/notification-recipient/delete
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes* / **Permission**: *write:admin:abuse-report:notification-recipient*
     */
    "admin___abuse-report___notification-recipient___delete": {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            id: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/accounts/create
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    admin___accounts___create: {
      requestBody: {
        content: {
          "application/json": {
            username: string,
            password: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["MeDetailed"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/accounts/delete
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:account*
     */
    admin___accounts___delete: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            userId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/accounts/find-by-email
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:admin:account*
     */
    "admin___accounts___find-by-email": {
      requestBody: {
        content: {
          "application/json": {
            email: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["UserDetailedNotMe"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/ad/create
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:ad*
     */
    admin___ad___create: {
      requestBody: {
        content: {
          "application/json": {
            url: string,
            memo: string,
            place: string,
            priority: string,
            ratio: number,
            expiresAt: number,
            startsAt: number,
            imageUrl: string,
            dayOfWeek: number,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Ad"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/ad/delete
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:ad*
     */
    admin___ad___delete: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            id: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/ad/list
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:admin:ad*
     */
    admin___ad___list: {
      requestBody: {
        content: {
          "application/json": {
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
            /** @default null */
            publishing?: boolean | null,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Ad"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/ad/update
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:ad*
     */
    admin___ad___update: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            id: string,
            memo?: string,
            url?: string,
            imageUrl?: string,
            place?: string,
            priority?: string,
            ratio?: number,
            expiresAt?: number,
            startsAt?: number,
            dayOfWeek?: number,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/announcements/create
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:announcements*
     */
    admin___announcements___create: {
      requestBody: {
        content: {
          "application/json": {
            title: string,
            text: string,
            imageUrl: string | null,
            /**
             * @default info
             * @enum {string}
             */
            icon?: "info" | "warning" | "error" | "success",
            /**
             * @default normal
             * @enum {string}
             */
            display?: "normal" | "banner" | "dialog",
            /** @default false */
            forExistingUsers?: boolean,
            /** @default false */
            silence?: boolean,
            /** @default false */
            needConfirmationToRead?: boolean,
            /**
             * Format: misskey:id
             * @default null
             */
            userId?: string | null,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              /**
               * Format: id
               * @example xxxxxxxxxx
               */
              id: string,
              /** Format: date-time */
              createdAt: string,
              /** Format: date-time */
              updatedAt: string | null,
              title: string,
              text: string,
              imageUrl: string | null,
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/announcements/delete
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:announcements*
     */
    admin___announcements___delete: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            id: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/announcements/list
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:admin:announcements*
     */
    admin___announcements___list: {
      requestBody: {
        content: {
          "application/json": {
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
            /** Format: misskey:id */
            userId?: string | null,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": ({
                /**
                 * Format: id
                 * @example xxxxxxxxxx
                 */
                id: string,
                /** Format: date-time */
                createdAt: string,
                /** Format: date-time */
                updatedAt: string | null,
                text: string,
                title: string,
                imageUrl: string | null,
                reads: number,
              })[],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/announcements/update
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:announcements*
     */
    admin___announcements___update: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            id: string,
            title?: string,
            text?: string,
            imageUrl?: string | null,
            /** @enum {string} */
            icon?: "info" | "warning" | "error" | "success",
            /** @enum {string} */
            display?: "normal" | "banner" | "dialog",
            forExistingUsers?: boolean,
            silence?: boolean,
            needConfirmationToRead?: boolean,
            isActive?: boolean,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/avatar-decorations/create
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:avatar-decorations*
     */
    "admin___avatar-decorations___create": {
      requestBody: {
        content: {
          "application/json": {
            name: string,
            description: string,
            url: string,
            roleIdsThatCanBeUsedThisDecoration?: string[],
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/avatar-decorations/delete
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:avatar-decorations*
     */
    "admin___avatar-decorations___delete": {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            id: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/avatar-decorations/list
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:admin:avatar-decorations*
     */
    "admin___avatar-decorations___list": {
      requestBody: {
        content: {
          "application/json": {
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
            /** Format: misskey:id */
            userId?: string | null,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": ({
                /**
                 * Format: id
                 * @example xxxxxxxxxx
                 */
                id: string,
                /** Format: date-time */
                createdAt: string,
                /** Format: date-time */
                updatedAt: string | null,
                name: string,
                description: string,
                url: string,
                roleIdsThatCanBeUsedThisDecoration: string[],
              })[],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/avatar-decorations/update
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:avatar-decorations*
     */
    "admin___avatar-decorations___update": {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            id: string,
            name?: string,
            description?: string,
            url?: string,
            roleIdsThatCanBeUsedThisDecoration?: string[],
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/delete-all-files-of-a-user
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:delete-all-files-of-a-user*
     */
    "admin___delete-all-files-of-a-user": {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            userId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/unset-user-avatar
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:unset-user-avatar*
     */
    "admin___unset-user-avatar": {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            userId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/unset-user-banner
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:unset-user-banner*
     */
    "admin___unset-user-banner": {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            userId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/drive/clean-remote-files
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:drive*
     */
    "admin___drive___clean-remote-files": {
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/drive/cleanup
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:drive*
     */
    admin___drive___cleanup: {
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/drive/files
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:admin:drive*
     */
    admin___drive___files: {
      requestBody: {
        content: {
          "application/json": {
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
            /** Format: misskey:id */
            userId?: string | null,
            type?: string | null,
            /**
             * @default local
             * @enum {string}
             */
            origin?: "combined" | "local" | "remote",
            /**
             * @description The local host is represented with `null`.
             * @default null
             */
            hostname?: string | null,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["DriveFile"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/drive/show-file
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:admin:drive*
     */
    "admin___drive___show-file": {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            fileId?: string,
            url?: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              /**
               * Format: id
               * @example xxxxxxxxxx
               */
              id: string,
              /** Format: date-time */
              createdAt: string,
              /**
               * Format: id
               * @example xxxxxxxxxx
               */
              userId: string | null,
              /** @description The local host is represented with `null`. */
              userHost: string | null,
              /**
               * Format: md5
               * @example 15eca7fba0480996e2245f5185bf39f2
               */
              md5: string,
              /** @example 192.jpg */
              name: string,
              /** @example image/jpeg */
              type: string,
              /** @example 51469 */
              size: number,
              comment: string | null,
              blurhash: string | null,
              properties: {
                width?: number,
                height?: number,
                orientation?: number,
                avgColor?: string,
              },
              /** @example true */
              storedInternal: boolean | null,
              /** Format: url */
              url: string | null,
              /** Format: url */
              thumbnailUrl: string | null,
              /** Format: url */
              webpublicUrl: string | null,
              accessKey: string | null,
              thumbnailAccessKey: string | null,
              webpublicAccessKey: string | null,
              uri: string | null,
              src: string | null,
              /**
               * Format: id
               * @example xxxxxxxxxx
               */
              folderId: string | null,
              isSensitive: boolean,
              isLink: boolean,
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/emoji/add-aliases-bulk
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:emoji*
     */
    "admin___emoji___add-aliases-bulk": {
      requestBody: {
        content: {
          "application/json": {
            ids: string[],
            aliases: string[],
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/emoji/add
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:emoji*
     */
    admin___emoji___add: {
      requestBody: {
        content: {
          "application/json": {
            name: string,
            /** Format: misskey:id */
            fileId: string,
            /** @description Use `null` to reset the category. */
            category?: string | null,
            aliases?: string[],
            license?: string | null,
            isSensitive?: boolean,
            localOnly?: boolean,
            roleIdsThatCanBeUsedThisEmojiAsReaction?: string[],
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["EmojiDetailed"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/emoji/copy
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:emoji*
     */
    admin___emoji___copy: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            emojiId: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              /** Format: id */
              id: string,
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/emoji/delete-bulk
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:emoji*
     */
    "admin___emoji___delete-bulk": {
      requestBody: {
        content: {
          "application/json": {
            ids: string[],
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/emoji/delete
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:emoji*
     */
    admin___emoji___delete: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            id: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/emoji/import-zip
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    "admin___emoji___import-zip": {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            fileId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/emoji/list-remote
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:admin:emoji*
     */
    "admin___emoji___list-remote": {
      requestBody: {
        content: {
          "application/json": {
            /** @default null */
            query?: string | null,
            /**
             * @description Use `null` to represent the local host.
             * @default null
             */
            host?: string | null,
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": ({
                /** Format: id */
                id: string,
                aliases: string[],
                name: string,
                category: string | null,
                /** @description The local host is represented with `null`. */
                host: string | null,
                url: string,
              })[],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/emoji/list
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:admin:emoji*
     */
    admin___emoji___list: {
      requestBody: {
        content: {
          "application/json": {
            /** @default null */
            query?: string | null,
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": ({
                /** Format: id */
                id: string,
                aliases: string[],
                name: string,
                category: string | null,
                /** @description The local host is represented with `null`. The field exists for compatibility with other API endpoints that return files. */
                host: string | null,
                url: string,
              })[],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/emoji/remove-aliases-bulk
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:emoji*
     */
    "admin___emoji___remove-aliases-bulk": {
      requestBody: {
        content: {
          "application/json": {
            ids: string[],
            aliases: string[],
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/emoji/set-aliases-bulk
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:emoji*
     */
    "admin___emoji___set-aliases-bulk": {
      requestBody: {
        content: {
          "application/json": {
            ids: string[],
            aliases: string[],
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/emoji/set-category-bulk
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:emoji*
     */
    "admin___emoji___set-category-bulk": {
      requestBody: {
        content: {
          "application/json": {
            ids: string[],
            /** @description Use `null` to reset the category. */
            category?: string | null,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/emoji/set-license-bulk
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:emoji*
     */
    "admin___emoji___set-license-bulk": {
      requestBody: {
        content: {
          "application/json": {
            ids: string[],
            /** @description Use `null` to reset the license. */
            license?: string | null,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/emoji/update
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:emoji*
     */
    admin___emoji___update: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            id?: string,
            name?: string,
            /** Format: misskey:id */
            fileId?: string,
            /** @description Use `null` to reset the category. */
            category?: string | null,
            aliases?: string[],
            license?: string | null,
            isSensitive?: boolean,
            localOnly?: boolean,
            roleIdsThatCanBeUsedThisEmojiAsReaction?: string[],
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/federation/delete-all-files
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:federation*
     */
    "admin___federation___delete-all-files": {
      requestBody: {
        content: {
          "application/json": {
            host: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/federation/refresh-remote-instance-metadata
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:federation*
     */
    "admin___federation___refresh-remote-instance-metadata": {
      requestBody: {
        content: {
          "application/json": {
            host: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/federation/remove-all-following
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:federation*
     */
    "admin___federation___remove-all-following": {
      requestBody: {
        content: {
          "application/json": {
            host: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/federation/update-instance
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:federation*
     */
    "admin___federation___update-instance": {
      requestBody: {
        content: {
          "application/json": {
            host: string,
            isSuspended?: boolean,
            moderationNote?: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/get-index-stats
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:admin:index-stats*
     */
    "admin___get-index-stats": {
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
                tablename: string,
                indexname: string,
              }[],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/get-table-stats
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:admin:table-stats*
     */
    "admin___get-table-stats": {
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              [key: string]: {
                count: number,
                size: number,
              },
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/get-user-ips
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:admin:user-ips*
     */
    "admin___get-user-ips": {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            userId: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
                ip: string,
                /** Format: date-time */
                createdAt: string,
              }[],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/invite/create
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:invite-codes*
     */
    admin___invite___create: {
      requestBody: {
        content: {
          "application/json": {
            /** @default 1 */
            count?: number,
            expiresAt?: string | null,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["InviteCode"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/invite/list
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:admin:invite-codes*
     */
    admin___invite___list: {
      requestBody: {
        content: {
          "application/json": {
            /** @default 30 */
            limit?: number,
            /** @default 0 */
            offset?: number,
            /**
             * @default all
             * @enum {string}
             */
            type?: "unused" | "used" | "expired" | "all",
            /** @enum {string} */
            sort?: "+createdAt" | "-createdAt" | "+usedAt" | "-usedAt",
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["InviteCode"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/promo/create
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:promo*
     */
    admin___promo___create: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            noteId: string,
            expiresAt: number,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/queue/clear
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:queue*
     */
    admin___queue___clear: {
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/queue/deliver-delayed
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:admin:queue*
     */
    "admin___queue___deliver-delayed": {
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": ((string | number)[])[],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/queue/inbox-delayed
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:admin:queue*
     */
    "admin___queue___inbox-delayed": {
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": ((string | number)[])[],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/queue/promote
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:queue*
     */
    admin___queue___promote: {
      requestBody: {
        content: {
          "application/json": {
            /** @enum {string} */
            type: "deliver" | "inbox",
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/queue/stats
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:admin:emoji*
     */
    admin___queue___stats: {
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              deliver: components["schemas"]["QueueCount"],
              inbox: components["schemas"]["QueueCount"],
              db: components["schemas"]["QueueCount"],
              objectStorage: components["schemas"]["QueueCount"],
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/relays/add
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:relays*
     */
    admin___relays___add: {
      requestBody: {
        content: {
          "application/json": {
            inbox: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              /** Format: id */
              id: string,
              /** Format: url */
              inbox: string,
              /**
               * @default requesting
               * @enum {string}
               */
              status: "requesting" | "accepted" | "rejected",
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/relays/list
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:admin:relays*
     */
    admin___relays___list: {
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": ({
                /** Format: id */
                id: string,
                /** Format: url */
                inbox: string,
                /**
                 * @default requesting
                 * @enum {string}
                 */
                status: "requesting" | "accepted" | "rejected",
              })[],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/relays/remove
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:relays*
     */
    admin___relays___remove: {
      requestBody: {
        content: {
          "application/json": {
            inbox: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/reset-password
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:reset-password*
     */
    "admin___reset-password": {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            userId: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              password: string,
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/resolve-abuse-user-report
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:resolve-abuse-user-report*
     */
    "admin___resolve-abuse-user-report": {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            reportId: string,
            /** @default false */
            forward?: boolean,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/send-email
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:send-email*
     */
    "admin___send-email": {
      requestBody: {
        content: {
          "application/json": {
            to: string,
            subject: string,
            text: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/server-info
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:admin:server-info*
     */
    "admin___server-info": {
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              machine: string,
              /** @example linux */
              os: string,
              node: string,
              psql: string,
              cpu: {
                model: string,
                cores: number,
              },
              mem: {
                /** Format: bytes */
                total: number,
              },
              fs: {
                /** Format: bytes */
                total: number,
                /** Format: bytes */
                used: number,
              },
              net: {
                /** @example eth0 */
                interface: string,
              },
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/show-moderation-logs
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:admin:show-moderation-log*
     */
    "admin___show-moderation-logs": {
      requestBody: {
        content: {
          "application/json": {
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
            type?: string | null,
            /** Format: misskey:id */
            userId?: string | null,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
                /** Format: id */
                id: string,
                /** Format: date-time */
                createdAt: string,
                type: string,
                info: Record<string, never>,
                /** Format: id */
                userId: string,
                user: components["schemas"]["UserDetailedNotMe"],
              }[],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/show-user
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:admin:show-user*
     */
    "admin___show-user": {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            userId: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              email: string | null,
              emailVerified: boolean,
              autoAcceptFollowed: boolean,
              noCrawle: boolean,
              preventAiLearning: boolean,
              alwaysMarkNsfw: boolean,
              autoSensitive: boolean,
              carefulBot: boolean,
              injectFeaturedNote: boolean,
              receiveAnnouncementEmail: boolean,
              mutedWords: (string | string[])[],
              mutedInstances: string[],
              notificationRecieveConfig: {
                note?: OneOf<[{
                  /** @enum {string} */
                  type: "all" | "following" | "follower" | "mutualFollow" | "followingOrFollower" | "never",
                }, {
                  /** @enum {string} */
                  type: "list",
                  /** Format: misskey:id */
                  userListId: string,
                }]>,
                follow?: OneOf<[{
                  /** @enum {string} */
                  type: "all" | "following" | "follower" | "mutualFollow" | "followingOrFollower" | "never",
                }, {
                  /** @enum {string} */
                  type: "list",
                  /** Format: misskey:id */
                  userListId: string,
                }]>,
                mention?: OneOf<[{
                  /** @enum {string} */
                  type: "all" | "following" | "follower" | "mutualFollow" | "followingOrFollower" | "never",
                }, {
                  /** @enum {string} */
                  type: "list",
                  /** Format: misskey:id */
                  userListId: string,
                }]>,
                reply?: OneOf<[{
                  /** @enum {string} */
                  type: "all" | "following" | "follower" | "mutualFollow" | "followingOrFollower" | "never",
                }, {
                  /** @enum {string} */
                  type: "list",
                  /** Format: misskey:id */
                  userListId: string,
                }]>,
                renote?: OneOf<[{
                  /** @enum {string} */
                  type: "all" | "following" | "follower" | "mutualFollow" | "followingOrFollower" | "never",
                }, {
                  /** @enum {string} */
                  type: "list",
                  /** Format: misskey:id */
                  userListId: string,
                }]>,
                quote?: OneOf<[{
                  /** @enum {string} */
                  type: "all" | "following" | "follower" | "mutualFollow" | "followingOrFollower" | "never",
                }, {
                  /** @enum {string} */
                  type: "list",
                  /** Format: misskey:id */
                  userListId: string,
                }]>,
                reaction?: OneOf<[{
                  /** @enum {string} */
                  type: "all" | "following" | "follower" | "mutualFollow" | "followingOrFollower" | "never",
                }, {
                  /** @enum {string} */
                  type: "list",
                  /** Format: misskey:id */
                  userListId: string,
                }]>,
                pollEnded?: OneOf<[{
                  /** @enum {string} */
                  type: "all" | "following" | "follower" | "mutualFollow" | "followingOrFollower" | "never",
                }, {
                  /** @enum {string} */
                  type: "list",
                  /** Format: misskey:id */
                  userListId: string,
                }]>,
                receiveFollowRequest?: OneOf<[{
                  /** @enum {string} */
                  type: "all" | "following" | "follower" | "mutualFollow" | "followingOrFollower" | "never",
                }, {
                  /** @enum {string} */
                  type: "list",
                  /** Format: misskey:id */
                  userListId: string,
                }]>,
                followRequestAccepted?: OneOf<[{
                  /** @enum {string} */
                  type: "all" | "following" | "follower" | "mutualFollow" | "followingOrFollower" | "never",
                }, {
                  /** @enum {string} */
                  type: "list",
                  /** Format: misskey:id */
                  userListId: string,
                }]>,
                roleAssigned?: OneOf<[{
                  /** @enum {string} */
                  type: "all" | "following" | "follower" | "mutualFollow" | "followingOrFollower" | "never",
                }, {
                  /** @enum {string} */
                  type: "list",
                  /** Format: misskey:id */
                  userListId: string,
                }]>,
                achievementEarned?: OneOf<[{
                  /** @enum {string} */
                  type: "all" | "following" | "follower" | "mutualFollow" | "followingOrFollower" | "never",
                }, {
                  /** @enum {string} */
                  type: "list",
                  /** Format: misskey:id */
                  userListId: string,
                }]>,
                app?: OneOf<[{
                  /** @enum {string} */
                  type: "all" | "following" | "follower" | "mutualFollow" | "followingOrFollower" | "never",
                }, {
                  /** @enum {string} */
                  type: "list",
                  /** Format: misskey:id */
                  userListId: string,
                }]>,
                test?: OneOf<[{
                  /** @enum {string} */
                  type: "all" | "following" | "follower" | "mutualFollow" | "followingOrFollower" | "never",
                }, {
                  /** @enum {string} */
                  type: "list",
                  /** Format: misskey:id */
                  userListId: string,
                }]>,
              },
              isModerator: boolean,
              isSilenced: boolean,
              isSuspended: boolean,
              isHibernated: boolean,
              lastActiveDate: string | null,
              moderationNote: string,
              signins: components["schemas"]["Signin"][],
              policies: components["schemas"]["RolePolicies"],
              roles: components["schemas"]["Role"][],
              roleAssigns: ({
                  createdAt: string,
                  expiresAt: string | null,
                  roleId: string,
                })[],
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/show-users
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:admin:show-user*
     */
    "admin___show-users": {
      requestBody: {
        content: {
          "application/json": {
            /** @default 10 */
            limit?: number,
            /** @default 0 */
            offset?: number,
            /** @enum {string} */
            sort?: "+follower" | "-follower" | "+createdAt" | "-createdAt" | "+updatedAt" | "-updatedAt" | "+lastActiveDate" | "-lastActiveDate",
            /**
             * @default all
             * @enum {string}
             */
            state?: "all" | "alive" | "available" | "admin" | "moderator" | "adminOrModerator" | "suspended",
            /**
             * @default combined
             * @enum {string}
             */
            origin?: "combined" | "local" | "remote",
            /** @default null */
            username?: string | null,
            /**
             * @description The local host is represented with `null`.
             * @default null
             */
            hostname?: string | null,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["UserDetailed"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/suspend-user
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:suspend-user*
     */
    "admin___suspend-user": {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            userId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/unsuspend-user
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:unsuspend-user*
     */
    "admin___unsuspend-user": {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            userId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/update-meta
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:meta*
     */
    "admin___update-meta": {
      requestBody: {
        content: {
          "application/json": {
            disableRegistration?: boolean | null,
            pinnedUsers?: string[] | null,
            hiddenTags?: string[] | null,
            blockedHosts?: string[] | null,
            sensitiveWords?: string[] | null,
            prohibitedWords?: string[] | null,
            themeColor?: string | null,
            mascotImageUrl?: string | null,
            bannerUrl?: string | null,
            serverErrorImageUrl?: string | null,
            infoImageUrl?: string | null,
            notFoundImageUrl?: string | null,
            iconUrl?: string | null,
            app192IconUrl?: string | null,
            app512IconUrl?: string | null,
            backgroundImageUrl?: string | null,
            logoImageUrl?: string | null,
            name?: string | null,
            shortName?: string | null,
            description?: string | null,
            defaultLightTheme?: string | null,
            defaultDarkTheme?: string | null,
            cacheRemoteFiles?: boolean,
            cacheRemoteSensitiveFiles?: boolean,
            emailRequiredForSignup?: boolean,
            enableHcaptcha?: boolean,
            hcaptchaSiteKey?: string | null,
            hcaptchaSecretKey?: string | null,
            enableMcaptcha?: boolean,
            mcaptchaSiteKey?: string | null,
            mcaptchaInstanceUrl?: string | null,
            mcaptchaSecretKey?: string | null,
            enableRecaptcha?: boolean,
            recaptchaSiteKey?: string | null,
            recaptchaSecretKey?: string | null,
            enableTurnstile?: boolean,
            turnstileSiteKey?: string | null,
            turnstileSecretKey?: string | null,
            /** @enum {string} */
            sensitiveMediaDetection?: "none" | "all" | "local" | "remote",
            /** @enum {string} */
            sensitiveMediaDetectionSensitivity?: "medium" | "low" | "high" | "veryLow" | "veryHigh",
            setSensitiveFlagAutomatically?: boolean,
            enableSensitiveMediaDetectionForVideos?: boolean,
            /** Format: misskey:id */
            proxyAccountId?: string | null,
            maintainerName?: string | null,
            maintainerEmail?: string | null,
            langs?: string[],
            deeplAuthKey?: string | null,
            deeplIsPro?: boolean,
            enableEmail?: boolean,
            email?: string | null,
            smtpSecure?: boolean,
            smtpHost?: string | null,
            smtpPort?: number | null,
            smtpUser?: string | null,
            smtpPass?: string | null,
            enableServiceWorker?: boolean,
            swPublicKey?: string | null,
            swPrivateKey?: string | null,
            tosUrl?: string | null,
            repositoryUrl?: string | null,
            feedbackUrl?: string | null,
            impressumUrl?: string | null,
            privacyPolicyUrl?: string | null,
            inquiryUrl?: string | null,
            useObjectStorage?: boolean,
            objectStorageBaseUrl?: string | null,
            objectStorageBucket?: string | null,
            objectStoragePrefix?: string | null,
            objectStorageEndpoint?: string | null,
            objectStorageRegion?: string | null,
            objectStoragePort?: number | null,
            objectStorageAccessKey?: string | null,
            objectStorageSecretKey?: string | null,
            objectStorageUseSSL?: boolean,
            objectStorageUseProxy?: boolean,
            objectStorageSetPublicRead?: boolean,
            objectStorageS3ForcePathStyle?: boolean,
            enableIpLogging?: boolean,
            enableActiveEmailValidation?: boolean,
            enableVerifymailApi?: boolean,
            verifymailAuthKey?: string | null,
            enableTruemailApi?: boolean,
            truemailInstance?: string | null,
            truemailAuthKey?: string | null,
            enableChartsForRemoteUser?: boolean,
            enableChartsForFederatedInstances?: boolean,
            enableServerMachineStats?: boolean,
            enableIdenticonGeneration?: boolean,
            serverRules?: string[],
            bannedEmailDomains?: string[],
            preservedUsernames?: string[],
            manifestJsonOverride?: string,
            enableFanoutTimeline?: boolean,
            enableFanoutTimelineDbFallback?: boolean,
            perLocalUserUserTimelineCacheMax?: number,
            perRemoteUserUserTimelineCacheMax?: number,
            perUserHomeTimelineCacheMax?: number,
            perUserListTimelineCacheMax?: number,
            notesPerOneAd?: number,
            silencedHosts?: string[] | null,
            /** @description [Deprecated] Use "urlPreviewSummaryProxyUrl" instead. */
            summalyProxy?: string | null,
            urlPreviewEnabled?: boolean,
            urlPreviewTimeout?: number,
            urlPreviewMaximumContentLength?: number,
            urlPreviewRequireContentLength?: boolean,
            urlPreviewUserAgent?: string | null,
            urlPreviewSummaryProxyUrl?: string | null,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/delete-account
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:delete-account*
     */
    "admin___delete-account": {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            userId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/update-user-note
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:user-note*
     */
    "admin___update-user-note": {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            userId: string,
            text: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/roles/create
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:roles*
     */
    admin___roles___create: {
      requestBody: {
        content: {
          "application/json": {
            name: string,
            description: string,
            color: string | null,
            iconUrl: string | null,
            /** @enum {string} */
            target: "manual" | "conditional",
            condFormula: Record<string, never>,
            isPublic: boolean,
            isModerator: boolean,
            isAdministrator: boolean,
            /** @default false */
            isExplorable?: boolean,
            asBadge: boolean,
            canEditMembersByModerator: boolean,
            displayOrder: number,
            policies: Record<string, never>,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Role"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/roles/delete
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:roles*
     */
    admin___roles___delete: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            roleId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/roles/list
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:admin:roles*
     */
    admin___roles___list: {
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Role"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/roles/show
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:admin:roles*
     */
    admin___roles___show: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            roleId: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Role"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/roles/update
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:roles*
     */
    admin___roles___update: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            roleId: string,
            name?: string,
            description?: string,
            color?: string | null,
            iconUrl?: string | null,
            /** @enum {string} */
            target?: "manual" | "conditional",
            condFormula?: Record<string, never>,
            isPublic?: boolean,
            isModerator?: boolean,
            isAdministrator?: boolean,
            isExplorable?: boolean,
            asBadge?: boolean,
            canEditMembersByModerator?: boolean,
            displayOrder?: number,
            policies?: Record<string, never>,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/roles/assign
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:roles*
     */
    admin___roles___assign: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            roleId: string,
            /** Format: misskey:id */
            userId: string,
            expiresAt?: number | null,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/roles/unassign
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:roles*
     */
    admin___roles___unassign: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            roleId: string,
            /** Format: misskey:id */
            userId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/roles/update-default-policies
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:admin:roles*
     */
    "admin___roles___update-default-policies": {
      requestBody: {
        content: {
          "application/json": {
            policies: Record<string, never>,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/roles/users
     * @description No description provided.
     *
     * **Credential required**: *No* / **Permission**: *read:admin:roles*
     */
    admin___roles___users: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            roleId: string,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
            /** @default 10 */
            limit?: number,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": ({
                /** Format: misskey:id */
                id: string,
                /** Format: date-time */
                createdAt: string,
                user: components["schemas"]["UserDetailed"],
                /** Format: date-time */
                expiresAt: string | null,
              })[],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/system-webhook/create
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes* / **Permission**: *write:admin:system-webhook*
     */
    "admin___system-webhook___create": {
      requestBody: {
        content: {
          "application/json": {
            isActive: boolean,
            name: string,
            on: ("abuseReport" | "abuseReportResolved")[],
            url: string,
            secret: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["SystemWebhook"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/system-webhook/delete
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes* / **Permission**: *write:admin:system-webhook*
     */
    "admin___system-webhook___delete": {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            id: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/system-webhook/list
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes* / **Permission**: *write:admin:system-webhook*
     */
    "admin___system-webhook___list": {
      requestBody: {
        content: {
          "application/json": {
            isActive?: boolean,
            on?: ("abuseReport" | "abuseReportResolved")[],
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["SystemWebhook"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/system-webhook/show
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes* / **Permission**: *write:admin:system-webhook*
     */
    "admin___system-webhook___show": {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            id: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["SystemWebhook"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * admin/system-webhook/update
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes* / **Permission**: *write:admin:system-webhook*
     */
    "admin___system-webhook___update": {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            id: string,
            isActive: boolean,
            name: string,
            on: ("abuseReport" | "abuseReportResolved")[],
            url: string,
            secret: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["SystemWebhook"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * announcements
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    announcements: {
      requestBody: {
        content: {
          "application/json": {
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
            /** @default true */
            isActive?: boolean,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Announcement"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * announcements/show
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    announcements___show: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            announcementId: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Announcement"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * antennas/create
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    antennas___create: {
      requestBody: {
        content: {
          "application/json": {
            name: string,
            /** @enum {string} */
            src: "home" | "all" | "users" | "list" | "users_blacklist",
            /** Format: misskey:id */
            userListId?: string | null,
            keywords: string[][],
            excludeKeywords: string[][],
            users: string[],
            caseSensitive: boolean,
            localOnly?: boolean,
            excludeBots?: boolean,
            withReplies: boolean,
            withFile: boolean,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Antenna"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * antennas/delete
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    antennas___delete: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            antennaId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * antennas/list
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    antennas___list: {
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Antenna"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * antennas/notes
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    antennas___notes: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            antennaId: string,
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
            sinceDate?: number,
            untilDate?: number,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Note"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * antennas/show
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    antennas___show: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            antennaId: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Antenna"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * antennas/update
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    antennas___update: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            antennaId: string,
            name?: string,
            /** @enum {string} */
            src?: "home" | "all" | "users" | "list" | "users_blacklist",
            /** Format: misskey:id */
            userListId?: string | null,
            keywords?: string[][],
            excludeKeywords?: string[][],
            users?: string[],
            caseSensitive?: boolean,
            localOnly?: boolean,
            excludeBots?: boolean,
            withReplies?: boolean,
            withFile?: boolean,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Antenna"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * ap/get
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:federation*
     */
    ap___get: {
      requestBody: {
        content: {
          "application/json": {
            uri: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": Record<string, never>,
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * ap/show
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    ap___show: {
      requestBody: {
        content: {
          "application/json": {
            uri: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": OneOf<[{
              /** @enum {string} */
              type: "User",
              object: components["schemas"]["UserDetailedNotMe"],
            }, {
              /** @enum {string} */
              type: "Note",
              object: components["schemas"]["Note"],
            }]>,
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * app/create
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    app___create: {
      requestBody: {
        content: {
          "application/json": {
            name: string,
            description: string,
            permission: string[],
            callbackUrl?: string | null,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["App"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * app/show
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    app___show: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            appId: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["App"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * auth/accept
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    auth___accept: {
      requestBody: {
        content: {
          "application/json": {
            token: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * auth/session/generate
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    auth___session___generate: {
      requestBody: {
        content: {
          "application/json": {
            appSecret: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              token: string,
              /** Format: url */
              url: string,
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * auth/session/show
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    auth___session___show: {
      requestBody: {
        content: {
          "application/json": {
            token: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              /** Format: id */
              id: string,
              app: components["schemas"]["App"],
              token: string,
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * auth/session/userkey
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    auth___session___userkey: {
      requestBody: {
        content: {
          "application/json": {
            appSecret: string,
            token: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              accessToken: string,
              user: components["schemas"]["UserDetailedNotMe"],
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * blocking/create
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:blocks*
     */
    blocking___create: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            userId: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["UserDetailedNotMe"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * blocking/delete
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:blocks*
     */
    blocking___delete: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            userId: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["UserDetailedNotMe"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * blocking/list
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:blocks*
     */
    blocking___list: {
      requestBody: {
        content: {
          "application/json": {
            /** @default 30 */
            limit?: number,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Blocking"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * channels/create
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:channels*
     */
    channels___create: {
      requestBody: {
        content: {
          "application/json": {
            name: string,
            description?: string | null,
            /** Format: misskey:id */
            bannerId?: string | null,
            color?: string,
            isSensitive?: boolean | null,
            allowRenoteToExternal?: boolean | null,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Channel"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * channels/featured
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    channels___featured: {
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Channel"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * channels/follow
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:channels*
     */
    channels___follow: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            channelId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * channels/followed
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:channels*
     */
    channels___followed: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
            /** @default 5 */
            limit?: number,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Channel"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * channels/owned
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:channels*
     */
    channels___owned: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
            /** @default 5 */
            limit?: number,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Channel"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * channels/show
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    channels___show: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            channelId: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Channel"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * channels/timeline
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    channels___timeline: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            channelId: string,
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
            sinceDate?: number,
            untilDate?: number,
            /** @default false */
            allowPartial?: boolean,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Note"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * channels/unfollow
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:channels*
     */
    channels___unfollow: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            channelId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * channels/update
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:channels*
     */
    channels___update: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            channelId: string,
            name?: string,
            description?: string | null,
            /** Format: misskey:id */
            bannerId?: string | null,
            isArchived?: boolean | null,
            pinnedNoteIds?: string[],
            color?: string,
            isSensitive?: boolean | null,
            allowRenoteToExternal?: boolean | null,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Channel"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * channels/favorite
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:channels*
     */
    channels___favorite: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            channelId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * channels/unfavorite
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:channels*
     */
    channels___unfavorite: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            channelId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * channels/my-favorites
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:channels*
     */
    "channels___my-favorites": {
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Channel"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * channels/search
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    channels___search: {
      requestBody: {
        content: {
          "application/json": {
            query: string,
            /**
             * @default nameAndDescription
             * @enum {string}
             */
            type?: "nameAndDescription" | "nameOnly",
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
            /** @default 5 */
            limit?: number,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Channel"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * charts/active-users
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    "charts___active-users": {
      requestBody: {
        content: {
          "application/json": {
            /** @enum {string} */
            span: "day" | "hour",
            /** @default 30 */
            limit?: number,
            /** @default null */
            offset?: number | null,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              readWrite: number[],
              read: number[],
              write: number[],
              registeredWithinWeek: number[],
              registeredWithinMonth: number[],
              registeredWithinYear: number[],
              registeredOutsideWeek: number[],
              registeredOutsideMonth: number[],
              registeredOutsideYear: number[],
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * charts/ap-request
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    "charts___ap-request": {
      requestBody: {
        content: {
          "application/json": {
            /** @enum {string} */
            span: "day" | "hour",
            /** @default 30 */
            limit?: number,
            /** @default null */
            offset?: number | null,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              deliverFailed: number[],
              deliverSucceeded: number[],
              inboxReceived: number[],
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * charts/drive
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    charts___drive: {
      requestBody: {
        content: {
          "application/json": {
            /** @enum {string} */
            span: "day" | "hour",
            /** @default 30 */
            limit?: number,
            /** @default null */
            offset?: number | null,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              local: {
                incCount: number[],
                incSize: number[],
                decCount: number[],
                decSize: number[],
              },
              remote: {
                incCount: number[],
                incSize: number[],
                decCount: number[],
                decSize: number[],
              },
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * charts/federation
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    charts___federation: {
      requestBody: {
        content: {
          "application/json": {
            /** @enum {string} */
            span: "day" | "hour",
            /** @default 30 */
            limit?: number,
            /** @default null */
            offset?: number | null,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              deliveredInstances: number[],
              inboxInstances: number[],
              stalled: number[],
              sub: number[],
              pub: number[],
              pubsub: number[],
              subActive: number[],
              pubActive: number[],
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * charts/instance
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    charts___instance: {
      requestBody: {
        content: {
          "application/json": {
            /** @enum {string} */
            span: "day" | "hour",
            /** @default 30 */
            limit?: number,
            /** @default null */
            offset?: number | null,
            host: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              requests: {
                failed: number[],
                succeeded: number[],
                received: number[],
              },
              notes: {
                total: number[],
                inc: number[],
                dec: number[],
                diffs: {
                  normal: number[],
                  reply: number[],
                  renote: number[],
                  withFile: number[],
                },
              },
              users: {
                total: number[],
                inc: number[],
                dec: number[],
              },
              following: {
                total: number[],
                inc: number[],
                dec: number[],
              },
              followers: {
                total: number[],
                inc: number[],
                dec: number[],
              },
              drive: {
                totalFiles: number[],
                incFiles: number[],
                decFiles: number[],
                incUsage: number[],
                decUsage: number[],
              },
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * charts/notes
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    charts___notes: {
      requestBody: {
        content: {
          "application/json": {
            /** @enum {string} */
            span: "day" | "hour",
            /** @default 30 */
            limit?: number,
            /** @default null */
            offset?: number | null,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              local: {
                total: number[],
                inc: number[],
                dec: number[],
                diffs: {
                  normal: number[],
                  reply: number[],
                  renote: number[],
                  withFile: number[],
                },
              },
              remote: {
                total: number[],
                inc: number[],
                dec: number[],
                diffs: {
                  normal: number[],
                  reply: number[],
                  renote: number[],
                  withFile: number[],
                },
              },
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * charts/user/drive
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    charts___user___drive: {
      requestBody: {
        content: {
          "application/json": {
            /** @enum {string} */
            span: "day" | "hour",
            /** @default 30 */
            limit?: number,
            /** @default null */
            offset?: number | null,
            /** Format: misskey:id */
            userId: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              totalCount: number[],
              totalSize: number[],
              incCount: number[],
              incSize: number[],
              decCount: number[],
              decSize: number[],
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * charts/user/following
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    charts___user___following: {
      requestBody: {
        content: {
          "application/json": {
            /** @enum {string} */
            span: "day" | "hour",
            /** @default 30 */
            limit?: number,
            /** @default null */
            offset?: number | null,
            /** Format: misskey:id */
            userId: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              local: {
                followings: {
                  total: number[],
                  inc: number[],
                  dec: number[],
                },
                followers: {
                  total: number[],
                  inc: number[],
                  dec: number[],
                },
              },
              remote: {
                followings: {
                  total: number[],
                  inc: number[],
                  dec: number[],
                },
                followers: {
                  total: number[],
                  inc: number[],
                  dec: number[],
                },
              },
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * charts/user/notes
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    charts___user___notes: {
      requestBody: {
        content: {
          "application/json": {
            /** @enum {string} */
            span: "day" | "hour",
            /** @default 30 */
            limit?: number,
            /** @default null */
            offset?: number | null,
            /** Format: misskey:id */
            userId: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              total: number[],
              inc: number[],
              dec: number[],
              diffs: {
                normal: number[],
                reply: number[],
                renote: number[],
                withFile: number[],
              },
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * charts/user/pv
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    charts___user___pv: {
      requestBody: {
        content: {
          "application/json": {
            /** @enum {string} */
            span: "day" | "hour",
            /** @default 30 */
            limit?: number,
            /** @default null */
            offset?: number | null,
            /** Format: misskey:id */
            userId: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              upv: {
                user: number[],
                visitor: number[],
              },
              pv: {
                user: number[],
                visitor: number[],
              },
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * charts/user/reactions
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    charts___user___reactions: {
      requestBody: {
        content: {
          "application/json": {
            /** @enum {string} */
            span: "day" | "hour",
            /** @default 30 */
            limit?: number,
            /** @default null */
            offset?: number | null,
            /** Format: misskey:id */
            userId: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              local: {
                count: number[],
              },
              remote: {
                count: number[],
              },
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * charts/users
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    charts___users: {
      requestBody: {
        content: {
          "application/json": {
            /** @enum {string} */
            span: "day" | "hour",
            /** @default 30 */
            limit?: number,
            /** @default null */
            offset?: number | null,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              local: {
                total: number[],
                inc: number[],
                dec: number[],
              },
              remote: {
                total: number[],
                inc: number[],
                dec: number[],
              },
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * clips/add-note
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    "clips___add-note": {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            clipId: string,
            /** Format: misskey:id */
            noteId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * clips/remove-note
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    "clips___remove-note": {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            clipId: string,
            /** Format: misskey:id */
            noteId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * clips/create
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    clips___create: {
      requestBody: {
        content: {
          "application/json": {
            name: string,
            /** @default false */
            isPublic?: boolean,
            description?: string | null,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Clip"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * clips/delete
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    clips___delete: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            clipId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * clips/list
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    clips___list: {
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Clip"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * clips/notes
     * @description No description provided.
     *
     * **Credential required**: *No* / **Permission**: *read:account*
     */
    clips___notes: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            clipId: string,
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Note"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * clips/show
     * @description No description provided.
     *
     * **Credential required**: *No* / **Permission**: *read:account*
     */
    clips___show: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            clipId: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Clip"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * clips/update
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    clips___update: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            clipId: string,
            name?: string,
            isPublic?: boolean,
            description?: string | null,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Clip"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * clips/favorite
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:clip-favorite*
     */
    clips___favorite: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            clipId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * clips/unfavorite
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:clip-favorite*
     */
    clips___unfavorite: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            clipId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * clips/my-favorites
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:clip-favorite*
     */
    "clips___my-favorites": {
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Clip"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * drive
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:drive*
     */
    drive: {
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              capacity: number,
              usage: number,
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * drive/files
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:drive*
     */
    drive___files: {
      requestBody: {
        content: {
          "application/json": {
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
            /**
             * Format: misskey:id
             * @default null
             */
            folderId?: string | null,
            type?: string | null,
            /** @enum {string|null} */
            sort?: "+createdAt" | "-createdAt" | "+name" | "-name" | "+size" | "-size" | null,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["DriveFile"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * drive/files/attached-notes
     * @description Find the notes to which the given file is attached.
     *
     * **Credential required**: *Yes* / **Permission**: *read:drive*
     */
    "drive___files___attached-notes": {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            fileId: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Note"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * drive/files/check-existence
     * @description Check if a given file exists.
     *
     * **Credential required**: *Yes* / **Permission**: *read:drive*
     */
    "drive___files___check-existence": {
      requestBody: {
        content: {
          "application/json": {
            md5: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": boolean,
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * drive/files/create
     * @description Upload a new drive file.
     *
     * **Credential required**: *Yes* / **Permission**: *write:drive*
     */
    drive___files___create: {
      requestBody: {
        content: {
          "multipart/form-data": {
            /**
             * Format: misskey:id
             * @default null
             */
            folderId?: string | null,
            /** @default null */
            name?: string | null,
            /** @default null */
            comment?: string | null,
            /** @default false */
            isSensitive?: boolean,
            /** @default false */
            force?: boolean,
            /**
             * Format: binary
             * @description The file contents.
             */
            file: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["DriveFile"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * drive/files/delete
     * @description Delete an existing drive file.
     *
     * **Credential required**: *Yes* / **Permission**: *write:drive*
     */
    drive___files___delete: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            fileId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * drive/files/find-by-hash
     * @description Search for a drive file by a hash of the contents.
     *
     * **Credential required**: *Yes* / **Permission**: *read:drive*
     */
    "drive___files___find-by-hash": {
      requestBody: {
        content: {
          "application/json": {
            md5: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["DriveFile"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * drive/files/find
     * @description Search for a drive file by the given parameters.
     *
     * **Credential required**: *Yes* / **Permission**: *read:drive*
     */
    drive___files___find: {
      requestBody: {
        content: {
          "application/json": {
            name: string,
            /**
             * Format: misskey:id
             * @default null
             */
            folderId?: string | null,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["DriveFile"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * drive/files/show
     * @description Show the properties of a drive file.
     *
     * **Credential required**: *Yes* / **Permission**: *read:drive*
     */
    drive___files___show: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            fileId?: string,
            url?: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["DriveFile"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * drive/files/update
     * @description Update the properties of a drive file.
     *
     * **Credential required**: *Yes* / **Permission**: *write:drive*
     */
    drive___files___update: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            fileId: string,
            /** Format: misskey:id */
            folderId?: string | null,
            name?: string,
            isSensitive?: boolean,
            comment?: string | null,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["DriveFile"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * drive/files/upload-from-url
     * @description Request the server to download a new drive file from the specified URL.
     *
     * **Credential required**: *Yes* / **Permission**: *write:drive*
     */
    "drive___files___upload-from-url": {
      requestBody: {
        content: {
          "application/json": {
            url: string,
            /**
             * Format: misskey:id
             * @default null
             */
            folderId?: string | null,
            /** @default false */
            isSensitive?: boolean,
            /** @default null */
            comment?: string | null,
            /** @default null */
            marker?: string | null,
            /** @default false */
            force?: boolean,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * drive/folders
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:drive*
     */
    drive___folders: {
      requestBody: {
        content: {
          "application/json": {
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
            /**
             * Format: misskey:id
             * @default null
             */
            folderId?: string | null,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["DriveFolder"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * drive/folders/create
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:drive*
     */
    drive___folders___create: {
      requestBody: {
        content: {
          "application/json": {
            /** @default Untitled */
            name?: string,
            /** Format: misskey:id */
            parentId?: string | null,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["DriveFolder"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * drive/folders/delete
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:drive*
     */
    drive___folders___delete: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            folderId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * drive/folders/find
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:drive*
     */
    drive___folders___find: {
      requestBody: {
        content: {
          "application/json": {
            name: string,
            /**
             * Format: misskey:id
             * @default null
             */
            parentId?: string | null,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["DriveFolder"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * drive/folders/show
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:drive*
     */
    drive___folders___show: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            folderId: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["DriveFolder"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * drive/folders/update
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:drive*
     */
    drive___folders___update: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            folderId: string,
            name?: string,
            /** Format: misskey:id */
            parentId?: string | null,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["DriveFolder"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * drive/stream
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:drive*
     */
    drive___stream: {
      requestBody: {
        content: {
          "application/json": {
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
            type?: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["DriveFile"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * email-address/available
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    "email-address___available": {
      requestBody: {
        content: {
          "application/json": {
            emailAddress: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              available: boolean,
              reason: string | null,
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * endpoint
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    endpoint: {
      requestBody: {
        content: {
          "application/json": {
            endpoint: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              params: {
                  name: string,
                  type: string,
                }[],
            } | null,
          },
        },
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * endpoints
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    endpoints: {
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": string[],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * export-custom-emojis
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    "export-custom-emojis": {
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * federation/followers
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    federation___followers: {
      requestBody: {
        content: {
          "application/json": {
            host: string,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
            /** @default 10 */
            limit?: number,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Following"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * federation/following
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    federation___following: {
      requestBody: {
        content: {
          "application/json": {
            host: string,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
            /** @default 10 */
            limit?: number,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Following"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * federation/instances
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    federation___instances: {
      requestBody: {
        content: {
          "application/json": {
            /** @description Omit or use `null` to not filter by host. */
            host?: string | null,
            blocked?: boolean | null,
            notResponding?: boolean | null,
            suspended?: boolean | null,
            silenced?: boolean | null,
            federating?: boolean | null,
            subscribing?: boolean | null,
            publishing?: boolean | null,
            /** @default 30 */
            limit?: number,
            /** @default 0 */
            offset?: number,
            /** @enum {string|null} */
            sort?: "+pubSub" | "-pubSub" | "+notes" | "-notes" | "+users" | "-users" | "+following" | "-following" | "+followers" | "-followers" | "+firstRetrievedAt" | "-firstRetrievedAt" | "+latestRequestReceivedAt" | "-latestRequestReceivedAt" | null,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["FederationInstance"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * federation/show-instance
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    "federation___show-instance": {
      requestBody: {
        content: {
          "application/json": {
            host: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["FederationInstance"] | null,
          },
        },
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * federation/update-remote-user
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    "federation___update-remote-user": {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            userId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * federation/users
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    federation___users: {
      requestBody: {
        content: {
          "application/json": {
            host: string,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
            /** @default 10 */
            limit?: number,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["UserDetailedNotMe"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * federation/stats
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    federation___stats: {
      requestBody: {
        content: {
          "application/json": {
            /** @default 10 */
            limit?: number,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              topSubInstances: components["schemas"]["FederationInstance"][],
              otherFollowersCount: number,
              topPubInstances: components["schemas"]["FederationInstance"][],
              otherFollowingCount: number,
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * following/create
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:following*
     */
    following___create: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            userId: string,
            withReplies?: boolean,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["UserLite"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * following/delete
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:following*
     */
    following___delete: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            userId: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["UserLite"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * following/update
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:following*
     */
    following___update: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            userId: string,
            /** @enum {string} */
            notify?: "normal" | "none",
            withReplies?: boolean,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["UserLite"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * following/update-all
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:following*
     */
    "following___update-all": {
      requestBody: {
        content: {
          "application/json": {
            /** @enum {string} */
            notify?: "normal" | "none",
            withReplies?: boolean,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * following/invalidate
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:following*
     */
    following___invalidate: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            userId: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["UserLite"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * following/requests/accept
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:following*
     */
    following___requests___accept: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            userId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * following/requests/cancel
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:following*
     */
    following___requests___cancel: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            userId: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["UserLite"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * following/requests/list
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:following*
     */
    following___requests___list: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
            /** @default 10 */
            limit?: number,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
                /** Format: id */
                id: string,
                follower: components["schemas"]["UserLite"],
                followee: components["schemas"]["UserLite"],
              }[],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * following/requests/reject
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:following*
     */
    following___requests___reject: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            userId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * gallery/featured
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    gallery___featured: {
      requestBody: {
        content: {
          "application/json": {
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            untilId?: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["GalleryPost"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * gallery/popular
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    gallery___popular: {
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["GalleryPost"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * gallery/posts
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    gallery___posts: {
      requestBody: {
        content: {
          "application/json": {
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["GalleryPost"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * gallery/posts/create
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:gallery*
     */
    gallery___posts___create: {
      requestBody: {
        content: {
          "application/json": {
            title: string,
            description?: string | null,
            fileIds: string[],
            /** @default false */
            isSensitive?: boolean,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["GalleryPost"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * gallery/posts/delete
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:gallery*
     */
    gallery___posts___delete: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            postId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * gallery/posts/like
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:gallery-likes*
     */
    gallery___posts___like: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            postId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * gallery/posts/show
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    gallery___posts___show: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            postId: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["GalleryPost"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * gallery/posts/unlike
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:gallery-likes*
     */
    gallery___posts___unlike: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            postId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * gallery/posts/update
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:gallery*
     */
    gallery___posts___update: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            postId: string,
            title?: string,
            description?: string | null,
            fileIds?: string[],
            /** @default false */
            isSensitive?: boolean,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["GalleryPost"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * get-online-users-count
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    "get-online-users-count": {
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              count: number,
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * get-avatar-decorations
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    "get-avatar-decorations": {
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
                /**
                 * Format: id
                 * @example xxxxxxxxxx
                 */
                id: string,
                name: string,
                description: string,
                url: string,
                roleIdsThatCanBeUsedThisDecoration: string[],
              }[],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * hashtags/list
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    hashtags___list: {
      requestBody: {
        content: {
          "application/json": {
            /** @default 10 */
            limit?: number,
            /** @default false */
            attachedToUserOnly?: boolean,
            /** @default false */
            attachedToLocalUserOnly?: boolean,
            /** @default false */
            attachedToRemoteUserOnly?: boolean,
            /** @enum {string} */
            sort: "+mentionedUsers" | "-mentionedUsers" | "+mentionedLocalUsers" | "-mentionedLocalUsers" | "+mentionedRemoteUsers" | "-mentionedRemoteUsers" | "+attachedUsers" | "-attachedUsers" | "+attachedLocalUsers" | "-attachedLocalUsers" | "+attachedRemoteUsers" | "-attachedRemoteUsers",
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Hashtag"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * hashtags/search
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    hashtags___search: {
      requestBody: {
        content: {
          "application/json": {
            /** @default 10 */
            limit?: number,
            query: string,
            /** @default 0 */
            offset?: number,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": string[],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * hashtags/show
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    hashtags___show: {
      requestBody: {
        content: {
          "application/json": {
            tag: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Hashtag"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * hashtags/trend
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    hashtags___trend: {
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
                tag: string,
                chart: number[],
                usersCount: number,
              }[],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * hashtags/users
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    hashtags___users: {
      requestBody: {
        content: {
          "application/json": {
            tag: string,
            /** @default 10 */
            limit?: number,
            /** @enum {string} */
            sort: "+follower" | "-follower" | "+createdAt" | "-createdAt" | "+updatedAt" | "-updatedAt",
            /**
             * @default all
             * @enum {string}
             */
            state?: "all" | "alive",
            /**
             * @default local
             * @enum {string}
             */
            origin?: "combined" | "local" | "remote",
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["UserDetailed"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    i: {
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["MeDetailed"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/2fa/done
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    i___2fa___done: {
      requestBody: {
        content: {
          "application/json": {
            token: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              backupCodes: string[],
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/2fa/key-done
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    "i___2fa___key-done": {
      requestBody: {
        content: {
          "application/json": {
            password: string,
            token?: string | null,
            name: string,
            credential: Record<string, never>,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              id: string,
              name: string,
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/2fa/password-less
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    "i___2fa___password-less": {
      requestBody: {
        content: {
          "application/json": {
            value: boolean,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/2fa/register-key
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    "i___2fa___register-key": {
      requestBody: {
        content: {
          "application/json": {
            password: string,
            token?: string | null,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              rp: {
                id?: string,
              },
              user: {
                id: string,
                name: string,
                displayName: string,
              },
              challenge: string,
              pubKeyCredParams: {
                  type: string,
                  alg: number,
                }[],
              timeout: number | null,
              excludeCredentials: (({
                  id: string,
                  type: string,
                  transports: ("ble" | "cable" | "hybrid" | "internal" | "nfc" | "smart-card" | "usb")[],
                })[]) | null,
              authenticatorSelection: ({
                /** @enum {string} */
                authenticatorAttachment: "cross-platform" | "platform",
                requireResidentKey: boolean,
                /** @enum {string} */
                userVerification: "discouraged" | "preferred" | "required",
              }) | null,
              /** @enum {string|null} */
              attestation: "direct" | "enterprise" | "indirect" | "none" | null,
              extensions: ({
                appid: string | null,
                credProps: boolean | null,
                hmacCreateSecret: boolean | null,
              }) | null,
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/2fa/register
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    i___2fa___register: {
      requestBody: {
        content: {
          "application/json": {
            password: string,
            token?: string | null,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              qr: string,
              url: string,
              secret: string,
              label: string,
              issuer: string,
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/2fa/update-key
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    "i___2fa___update-key": {
      requestBody: {
        content: {
          "application/json": {
            name: string,
            credentialId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/2fa/remove-key
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    "i___2fa___remove-key": {
      requestBody: {
        content: {
          "application/json": {
            password: string,
            token?: string | null,
            credentialId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/2fa/unregister
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    i___2fa___unregister: {
      requestBody: {
        content: {
          "application/json": {
            password: string,
            token?: string | null,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/apps
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    i___apps: {
      requestBody: {
        content: {
          "application/json": {
            /** @enum {string} */
            sort?: "+createdAt" | "-createdAt" | "+lastUsedAt" | "-lastUsedAt",
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
                /** Format: misskey:id */
                id: string,
                name?: string,
                /** Format: date-time */
                createdAt: string,
                /** Format: date-time */
                lastUsedAt?: string,
                permission: string[],
              }[],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/authorized-apps
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    "i___authorized-apps": {
      requestBody: {
        content: {
          "application/json": {
            /** @default 10 */
            limit?: number,
            /** @default 0 */
            offset?: number,
            /**
             * @default desc
             * @enum {string}
             */
            sort?: "desc" | "asc",
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": ({
                /** Format: misskey:id */
                id: string,
                name: string,
                callbackUrl: string | null,
                permission: string[],
                isAuthorized?: boolean,
              })[],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/claim-achievement
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    "i___claim-achievement": {
      requestBody: {
        content: {
          "application/json": {
            /** @enum {string} */
            name: "notes1" | "notes10" | "notes100" | "notes500" | "notes1000" | "notes5000" | "notes10000" | "notes20000" | "notes30000" | "notes40000" | "notes50000" | "notes60000" | "notes70000" | "notes80000" | "notes90000" | "notes100000" | "login3" | "login7" | "login15" | "login30" | "login60" | "login100" | "login200" | "login300" | "login400" | "login500" | "login600" | "login700" | "login800" | "login900" | "login1000" | "passedSinceAccountCreated1" | "passedSinceAccountCreated2" | "passedSinceAccountCreated3" | "loggedInOnBirthday" | "loggedInOnNewYearsDay" | "noteClipped1" | "noteFavorited1" | "myNoteFavorited1" | "profileFilled" | "markedAsCat" | "following1" | "following10" | "following50" | "following100" | "following300" | "followers1" | "followers10" | "followers50" | "followers100" | "followers300" | "followers500" | "followers1000" | "collectAchievements30" | "viewAchievements3min" | "iLoveMisskey" | "foundTreasure" | "client30min" | "client60min" | "noteDeletedWithin1min" | "postedAtLateNight" | "postedAt0min0sec" | "selfQuote" | "htl20npm" | "viewInstanceChart" | "outputHelloWorldOnScratchpad" | "open3windows" | "driveFolderCircularReference" | "reactWithoutRead" | "clickedClickHere" | "justPlainLucky" | "setNameToSyuilo" | "cookieClicked" | "brainDiver" | "smashTestNotificationButton" | "tutorialCompleted" | "bubbleGameExplodingHead" | "bubbleGameDoubleExplodingHead",
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/change-password
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    "i___change-password": {
      requestBody: {
        content: {
          "application/json": {
            currentPassword: string,
            newPassword: string,
            token?: string | null,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/delete-account
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    "i___delete-account": {
      requestBody: {
        content: {
          "application/json": {
            password: string,
            token?: string | null,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/export-blocking
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    "i___export-blocking": {
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/export-following
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    "i___export-following": {
      requestBody: {
        content: {
          "application/json": {
            /** @default false */
            excludeMuting?: boolean,
            /** @default false */
            excludeInactive?: boolean,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/export-mute
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    "i___export-mute": {
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/export-notes
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    "i___export-notes": {
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/export-clips
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    "i___export-clips": {
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/export-favorites
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    "i___export-favorites": {
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/export-user-lists
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    "i___export-user-lists": {
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/export-antennas
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    "i___export-antennas": {
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/favorites
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:favorites*
     */
    i___favorites: {
      requestBody: {
        content: {
          "application/json": {
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["NoteFavorite"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/gallery/likes
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:gallery-likes*
     */
    i___gallery___likes: {
      requestBody: {
        content: {
          "application/json": {
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
                /** Format: id */
                id: string,
                post: components["schemas"]["GalleryPost"],
              }[],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/gallery/posts
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:gallery*
     */
    i___gallery___posts: {
      requestBody: {
        content: {
          "application/json": {
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["GalleryPost"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/import-blocking
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    "i___import-blocking": {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            fileId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/import-following
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    "i___import-following": {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            fileId: string,
            withReplies?: boolean,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/import-muting
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    "i___import-muting": {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            fileId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/import-user-lists
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    "i___import-user-lists": {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            fileId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/import-antennas
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    "i___import-antennas": {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            fileId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/notifications
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:notifications*
     */
    i___notifications: {
      requestBody: {
        content: {
          "application/json": {
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
            /** @default true */
            markAsRead?: boolean,
            includeTypes?: ("note" | "follow" | "mention" | "reply" | "renote" | "quote" | "reaction" | "pollEnded" | "receiveFollowRequest" | "followRequestAccepted" | "roleAssigned" | "achievementEarned" | "app" | "test" | "pollVote" | "groupInvited")[],
            excludeTypes?: ("note" | "follow" | "mention" | "reply" | "renote" | "quote" | "reaction" | "pollEnded" | "receiveFollowRequest" | "followRequestAccepted" | "roleAssigned" | "achievementEarned" | "app" | "test" | "pollVote" | "groupInvited")[],
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Notification"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/notifications-grouped
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:notifications*
     */
    "i___notifications-grouped": {
      requestBody: {
        content: {
          "application/json": {
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
            /** @default true */
            markAsRead?: boolean,
            includeTypes?: ("note" | "follow" | "mention" | "reply" | "renote" | "quote" | "reaction" | "pollEnded" | "receiveFollowRequest" | "followRequestAccepted" | "roleAssigned" | "achievementEarned" | "app" | "test" | "reaction:grouped" | "renote:grouped" | "pollVote" | "groupInvited")[],
            excludeTypes?: ("note" | "follow" | "mention" | "reply" | "renote" | "quote" | "reaction" | "pollEnded" | "receiveFollowRequest" | "followRequestAccepted" | "roleAssigned" | "achievementEarned" | "app" | "test" | "reaction:grouped" | "renote:grouped" | "pollVote" | "groupInvited")[],
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Notification"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/page-likes
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:page-likes*
     */
    "i___page-likes": {
      requestBody: {
        content: {
          "application/json": {
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
                /** Format: id */
                id: string,
                page: components["schemas"]["Page"],
              }[],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/pages
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:pages*
     */
    i___pages: {
      requestBody: {
        content: {
          "application/json": {
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Page"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/pin
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    i___pin: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            noteId: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["MeDetailed"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/read-all-unread-notes
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    "i___read-all-unread-notes": {
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/read-announcement
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    "i___read-announcement": {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            announcementId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/regenerate-token
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    "i___regenerate-token": {
      requestBody: {
        content: {
          "application/json": {
            password: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/registry/get-all
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    "i___registry___get-all": {
      requestBody: {
        content: {
          "application/json": {
            /** @default [] */
            scope: string[],
            domain?: string | null,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": Record<string, never>,
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/registry/get-detail
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    "i___registry___get-detail": {
      requestBody: {
        content: {
          "application/json": {
            key: string,
            /** @default [] */
            scope: string[],
            domain?: string | null,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              updatedAt: string,
              value: unknown,
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/registry/get
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    i___registry___get: {
      requestBody: {
        content: {
          "application/json": {
            key: string,
            /** @default [] */
            scope: string[],
            domain?: string | null,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": Record<string, never>,
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/registry/keys-with-type
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    "i___registry___keys-with-type": {
      requestBody: {
        content: {
          "application/json": {
            /** @default [] */
            scope: string[],
            domain?: string | null,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              [key: string]: string,
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/registry/keys
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    i___registry___keys: {
      requestBody: {
        content: {
          "application/json": {
            /** @default [] */
            scope: string[],
            domain?: string | null,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": string[],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/registry/remove
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    i___registry___remove: {
      requestBody: {
        content: {
          "application/json": {
            key: string,
            /** @default [] */
            scope: string[],
            domain?: string | null,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/registry/scopes-with-domain
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    "i___registry___scopes-with-domain": {
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": ({
                scopes: string[][],
                domain: string | null,
              })[],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/registry/set
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    i___registry___set: {
      requestBody: {
        content: {
          "application/json": {
            key: string,
            value: unknown,
            /** @default [] */
            scope: string[],
            domain?: string | null,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/revoke-token
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    "i___revoke-token": {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            tokenId?: string,
            token?: string | null,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/signin-history
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    "i___signin-history": {
      requestBody: {
        content: {
          "application/json": {
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Signin"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/unpin
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    i___unpin: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            noteId: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["MeDetailed"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/update-email
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    "i___update-email": {
      requestBody: {
        content: {
          "application/json": {
            password: string,
            email?: string | null,
            token?: string | null,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["MeDetailed"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/update
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    i___update: {
      requestBody: {
        content: {
          "application/json": {
            name?: string | null,
            description?: string | null,
            location?: string | null,
            birthday?: string | null,
            /** @enum {string|null} */
            lang?: null | "ach" | "ady" | "af" | "af-NA" | "af-ZA" | "ak" | "ar" | "ar-AR" | "ar-MA" | "ar-SA" | "ay-BO" | "az" | "az-AZ" | "be-BY" | "bg" | "bg-BG" | "bn" | "bn-IN" | "bn-BD" | "br" | "bs-BA" | "ca" | "ca-ES" | "cak" | "ck-US" | "cs" | "cs-CZ" | "cy" | "cy-GB" | "da" | "da-DK" | "de" | "de-AT" | "de-DE" | "de-CH" | "dsb" | "el" | "el-GR" | "en" | "en-GB" | "en-AU" | "en-CA" | "en-IE" | "en-IN" | "en-PI" | "en-SG" | "en-UD" | "en-US" | "en-ZA" | "en@pirate" | "eo" | "eo-EO" | "es" | "es-AR" | "es-419" | "es-CL" | "es-CO" | "es-EC" | "es-ES" | "es-LA" | "es-NI" | "es-MX" | "es-US" | "es-VE" | "et" | "et-EE" | "eu" | "eu-ES" | "fa" | "fa-IR" | "fb-LT" | "ff" | "fi" | "fi-FI" | "fo" | "fo-FO" | "fr" | "fr-CA" | "fr-FR" | "fr-BE" | "fr-CH" | "fy-NL" | "ga" | "ga-IE" | "gd" | "gl" | "gl-ES" | "gn-PY" | "gu-IN" | "gv" | "gx-GR" | "he" | "he-IL" | "hi" | "hi-IN" | "hr" | "hr-HR" | "hsb" | "ht" | "hu" | "hu-HU" | "hy" | "hy-AM" | "id" | "id-ID" | "is" | "is-IS" | "it" | "it-IT" | "ja" | "ja-JP" | "jv-ID" | "ka-GE" | "kk-KZ" | "km" | "kl" | "km-KH" | "kab" | "kn" | "kn-IN" | "ko" | "ko-KR" | "ku-TR" | "kw" | "la" | "la-VA" | "lb" | "li-NL" | "lt" | "lt-LT" | "lv" | "lv-LV" | "mai" | "mg-MG" | "mk" | "mk-MK" | "ml" | "ml-IN" | "mn-MN" | "mr" | "mr-IN" | "ms" | "ms-MY" | "mt" | "mt-MT" | "my" | "no" | "nb" | "nb-NO" | "ne" | "ne-NP" | "nl" | "nl-BE" | "nl-NL" | "nn-NO" | "oc" | "or-IN" | "pa" | "pa-IN" | "pl" | "pl-PL" | "ps-AF" | "pt" | "pt-BR" | "pt-PT" | "qu-PE" | "rm-CH" | "ro" | "ro-RO" | "ru" | "ru-RU" | "sa-IN" | "se-NO" | "sh" | "si-LK" | "sk" | "sk-SK" | "sl" | "sl-SI" | "so-SO" | "sq" | "sq-AL" | "sr" | "sr-RS" | "su" | "sv" | "sv-SE" | "sw" | "sw-KE" | "ta" | "ta-IN" | "te" | "te-IN" | "tg" | "tg-TJ" | "th" | "th-TH" | "fil" | "tlh" | "tr" | "tr-TR" | "tt-RU" | "uk" | "uk-UA" | "ur" | "ur-PK" | "uz" | "uz-UZ" | "vi" | "vi-VN" | "xh-ZA" | "yi" | "yi-DE" | "zh" | "zh-Hans" | "zh-Hant" | "zh-CN" | "zh-HK" | "zh-SG" | "zh-TW" | "zu-ZA",
            /** Format: misskey:id */
            avatarId?: string | null,
            avatarDecorations?: ({
                /** Format: misskey:id */
                id: string,
                angle?: number | null,
                flipH?: boolean | null,
                offsetX?: number | null,
                offsetY?: number | null,
              })[],
            /** Format: misskey:id */
            bannerId?: string | null,
            fields?: {
                name: string,
                value: string,
              }[],
            isLocked?: boolean,
            isExplorable?: boolean,
            hideOnlineStatus?: boolean,
            publicReactions?: boolean,
            carefulBot?: boolean,
            autoAcceptFollowed?: boolean,
            noCrawle?: boolean,
            preventAiLearning?: boolean,
            isBot?: boolean,
            isCat?: boolean,
            injectFeaturedNote?: boolean,
            receiveAnnouncementEmail?: boolean,
            alwaysMarkNsfw?: boolean,
            autoSensitive?: boolean,
            /** @enum {string} */
            followingVisibility?: "public" | "followers" | "private",
            /** @enum {string} */
            followersVisibility?: "public" | "followers" | "private",
            /** Format: misskey:id */
            pinnedPageId?: string | null,
            mutedWords?: (string[] | string)[],
            hardMutedWords?: (string[] | string)[],
            mutedInstances?: string[],
            notificationRecieveConfig?: {
              note?: OneOf<[{
                /** @enum {string} */
                type: "all" | "following" | "follower" | "mutualFollow" | "followingOrFollower" | "never",
              }, {
                /** @enum {string} */
                type: "list",
                /** Format: misskey:id */
                userListId: string,
              }]>,
              follow?: OneOf<[{
                /** @enum {string} */
                type: "all" | "following" | "follower" | "mutualFollow" | "followingOrFollower" | "never",
              }, {
                /** @enum {string} */
                type: "list",
                /** Format: misskey:id */
                userListId: string,
              }]>,
              mention?: OneOf<[{
                /** @enum {string} */
                type: "all" | "following" | "follower" | "mutualFollow" | "followingOrFollower" | "never",
              }, {
                /** @enum {string} */
                type: "list",
                /** Format: misskey:id */
                userListId: string,
              }]>,
              reply?: OneOf<[{
                /** @enum {string} */
                type: "all" | "following" | "follower" | "mutualFollow" | "followingOrFollower" | "never",
              }, {
                /** @enum {string} */
                type: "list",
                /** Format: misskey:id */
                userListId: string,
              }]>,
              renote?: OneOf<[{
                /** @enum {string} */
                type: "all" | "following" | "follower" | "mutualFollow" | "followingOrFollower" | "never",
              }, {
                /** @enum {string} */
                type: "list",
                /** Format: misskey:id */
                userListId: string,
              }]>,
              quote?: OneOf<[{
                /** @enum {string} */
                type: "all" | "following" | "follower" | "mutualFollow" | "followingOrFollower" | "never",
              }, {
                /** @enum {string} */
                type: "list",
                /** Format: misskey:id */
                userListId: string,
              }]>,
              reaction?: OneOf<[{
                /** @enum {string} */
                type: "all" | "following" | "follower" | "mutualFollow" | "followingOrFollower" | "never",
              }, {
                /** @enum {string} */
                type: "list",
                /** Format: misskey:id */
                userListId: string,
              }]>,
              pollEnded?: OneOf<[{
                /** @enum {string} */
                type: "all" | "following" | "follower" | "mutualFollow" | "followingOrFollower" | "never",
              }, {
                /** @enum {string} */
                type: "list",
                /** Format: misskey:id */
                userListId: string,
              }]>,
              receiveFollowRequest?: OneOf<[{
                /** @enum {string} */
                type: "all" | "following" | "follower" | "mutualFollow" | "followingOrFollower" | "never",
              }, {
                /** @enum {string} */
                type: "list",
                /** Format: misskey:id */
                userListId: string,
              }]>,
              followRequestAccepted?: OneOf<[{
                /** @enum {string} */
                type: "all" | "following" | "follower" | "mutualFollow" | "followingOrFollower" | "never",
              }, {
                /** @enum {string} */
                type: "list",
                /** Format: misskey:id */
                userListId: string,
              }]>,
              roleAssigned?: OneOf<[{
                /** @enum {string} */
                type: "all" | "following" | "follower" | "mutualFollow" | "followingOrFollower" | "never",
              }, {
                /** @enum {string} */
                type: "list",
                /** Format: misskey:id */
                userListId: string,
              }]>,
              achievementEarned?: OneOf<[{
                /** @enum {string} */
                type: "all" | "following" | "follower" | "mutualFollow" | "followingOrFollower" | "never",
              }, {
                /** @enum {string} */
                type: "list",
                /** Format: misskey:id */
                userListId: string,
              }]>,
              app?: OneOf<[{
                /** @enum {string} */
                type: "all" | "following" | "follower" | "mutualFollow" | "followingOrFollower" | "never",
              }, {
                /** @enum {string} */
                type: "list",
                /** Format: misskey:id */
                userListId: string,
              }]>,
              test?: OneOf<[{
                /** @enum {string} */
                type: "all" | "following" | "follower" | "mutualFollow" | "followingOrFollower" | "never",
              }, {
                /** @enum {string} */
                type: "list",
                /** Format: misskey:id */
                userListId: string,
              }]>,
            },
            emailNotificationTypes?: string[],
            alsoKnownAs?: string[],
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["MeDetailed"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/move
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    i___move: {
      requestBody: {
        content: {
          "application/json": {
            moveToAccount: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": Record<string, never>,
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/webhooks/create
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    i___webhooks___create: {
      requestBody: {
        content: {
          "application/json": {
            name: string,
            url: string,
            /** @default */
            secret?: string,
            on: ("mention" | "unfollow" | "follow" | "followed" | "note" | "reply" | "renote" | "reaction")[],
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              /** Format: misskey:id */
              id: string,
              /** Format: misskey:id */
              userId: string,
              name: string,
              on: ("mention" | "unfollow" | "follow" | "followed" | "note" | "reply" | "renote" | "reaction")[],
              url: string,
              secret: string,
              active: boolean,
              /** Format: date-time */
              latestSentAt: string | null,
              latestStatus: number | null,
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/webhooks/list
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    i___webhooks___list: {
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": ({
                /** Format: misskey:id */
                id: string,
                /** Format: misskey:id */
                userId: string,
                name: string,
                on: ("mention" | "unfollow" | "follow" | "followed" | "note" | "reply" | "renote" | "reaction")[],
                url: string,
                secret: string,
                active: boolean,
                /** Format: date-time */
                latestSentAt: string | null,
                latestStatus: number | null,
              })[],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/webhooks/show
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    i___webhooks___show: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            webhookId: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              /** Format: misskey:id */
              id: string,
              /** Format: misskey:id */
              userId: string,
              name: string,
              on: ("mention" | "unfollow" | "follow" | "followed" | "note" | "reply" | "renote" | "reaction")[],
              url: string,
              secret: string,
              active: boolean,
              /** Format: date-time */
              latestSentAt: string | null,
              latestStatus: number | null,
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/webhooks/update
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    i___webhooks___update: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            webhookId: string,
            name?: string,
            url?: string,
            secret?: string | null,
            on?: ("mention" | "unfollow" | "follow" | "followed" | "note" | "reply" | "renote" | "reaction")[],
            active?: boolean,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * i/webhooks/delete
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    i___webhooks___delete: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            webhookId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * invite/create
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:invite-codes*
     */
    invite___create: {
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["InviteCode"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * invite/delete
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:invite-codes*
     */
    invite___delete: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            inviteId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * invite/list
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:invite-codes*
     */
    invite___list: {
      requestBody: {
        content: {
          "application/json": {
            /** @default 30 */
            limit?: number,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["InviteCode"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * invite/limit
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:invite-codes*
     */
    invite___limit: {
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              remaining: number | null,
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * meta
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    meta: {
      requestBody: {
        content: {
          "application/json": {
            /** @default true */
            detail?: boolean,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["MetaLite"] | components["schemas"]["MetaDetailed"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * emojis
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    emojis: {
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              emojis: components["schemas"]["EmojiSimple"][],
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * emoji
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    emoji: {
      requestBody: {
        content: {
          "application/json": {
            name: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["EmojiDetailed"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * miauth/gen-token
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    "miauth___gen-token": {
      requestBody: {
        content: {
          "application/json": {
            session: string | null,
            name?: string | null,
            description?: string | null,
            iconUrl?: string | null,
            permission: string[],
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              token: string,
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * mute/create
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:mutes*
     */
    mute___create: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            userId: string,
            /** @description A Unix Epoch timestamp that must lie in the future. `null` means an indefinite mute. */
            expiresAt?: number | null,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * mute/delete
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:mutes*
     */
    mute___delete: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            userId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * mute/list
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:mutes*
     */
    mute___list: {
      requestBody: {
        content: {
          "application/json": {
            /** @default 30 */
            limit?: number,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Muting"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * renote-mute/create
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:mutes*
     */
    "renote-mute___create": {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            userId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * renote-mute/delete
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:mutes*
     */
    "renote-mute___delete": {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            userId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * renote-mute/list
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:mutes*
     */
    "renote-mute___list": {
      requestBody: {
        content: {
          "application/json": {
            /** @default 30 */
            limit?: number,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["RenoteMuting"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * my/apps
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    my___apps: {
      requestBody: {
        content: {
          "application/json": {
            /** @default 10 */
            limit?: number,
            /** @default 0 */
            offset?: number,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["App"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * notes
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    notes: {
      requestBody: {
        content: {
          "application/json": {
            /** @default false */
            local?: boolean,
            reply?: boolean,
            renote?: boolean,
            withFiles?: boolean,
            poll?: boolean,
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Note"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * notes/children
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    notes___children: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            noteId: string,
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Note"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * notes/clips
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    notes___clips: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            noteId: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Clip"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * notes/conversation
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    notes___conversation: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            noteId: string,
            /** @default 10 */
            limit?: number,
            /** @default 0 */
            offset?: number,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Note"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * notes/create
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:notes*
     */
    notes___create: {
      requestBody: {
        content: {
          "application/json": {
            /**
             * @default public
             * @enum {string}
             */
            visibility?: "public" | "home" | "followers" | "specified",
            visibleUserIds?: string[],
            cw?: string | null,
            /** @default false */
            localOnly?: boolean,
            /**
             * @default null
             * @enum {string|null}
             */
            reactionAcceptance?: null | "likeOnly" | "likeOnlyForRemote" | "nonSensitiveOnly" | "nonSensitiveOnlyForLocalLikeOnlyForRemote",
            /** @default false */
            noExtractMentions?: boolean,
            /** @default false */
            noExtractHashtags?: boolean,
            /** @default false */
            noExtractEmojis?: boolean,
            /** Format: misskey:id */
            replyId?: string | null,
            /** Format: misskey:id */
            renoteId?: string | null,
            /** Format: misskey:id */
            channelId?: string | null,
            text?: string | null,
            fileIds?: string[],
            mediaIds?: string[],
            poll?: ({
              choices: string[],
              multiple?: boolean,
              expiresAt?: number | null,
              expiredAfter?: number | null,
            }) | null,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              createdNote: components["schemas"]["Note"],
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * notes/delete
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:notes*
     */
    notes___delete: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            noteId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * notes/favorites/create
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:favorites*
     */
    notes___favorites___create: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            noteId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * notes/favorites/delete
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:favorites*
     */
    notes___favorites___delete: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            noteId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * notes/featured
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    notes___featured: {
      requestBody: {
        content: {
          "application/json": {
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            untilId?: string,
            /** Format: misskey:id */
            channelId?: string | null,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Note"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * notes/global-timeline
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    "notes___global-timeline": {
      requestBody: {
        content: {
          "application/json": {
            /** @default false */
            withFiles?: boolean,
            /** @default true */
            withRenotes?: boolean,
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
            sinceDate?: number,
            untilDate?: number,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Note"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * notes/hybrid-timeline
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    "notes___hybrid-timeline": {
      requestBody: {
        content: {
          "application/json": {
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
            sinceDate?: number,
            untilDate?: number,
            /** @default false */
            allowPartial?: boolean,
            /** @default true */
            includeMyRenotes?: boolean,
            /** @default true */
            includeRenotedMyNotes?: boolean,
            /** @default true */
            includeLocalRenotes?: boolean,
            /** @default false */
            withFiles?: boolean,
            /** @default true */
            withRenotes?: boolean,
            /** @default false */
            withReplies?: boolean,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Note"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * notes/local-timeline
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    "notes___local-timeline": {
      requestBody: {
        content: {
          "application/json": {
            /** @default false */
            withFiles?: boolean,
            /** @default true */
            withRenotes?: boolean,
            /** @default false */
            withReplies?: boolean,
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
            /** @default false */
            allowPartial?: boolean,
            sinceDate?: number,
            untilDate?: number,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Note"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * notes/mentions
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    notes___mentions: {
      requestBody: {
        content: {
          "application/json": {
            /** @default false */
            following?: boolean,
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
            visibility?: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Note"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * notes/polls/recommendation
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    notes___polls___recommendation: {
      requestBody: {
        content: {
          "application/json": {
            /** @default 10 */
            limit?: number,
            /** @default 0 */
            offset?: number,
            /** @default false */
            excludeChannels?: boolean,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Note"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * notes/polls/vote
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:votes*
     */
    notes___polls___vote: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            noteId: string,
            choice: number,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * notes/reactions
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    notes___reactions: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            noteId: string,
            type?: string | null,
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["NoteReaction"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * notes/reactions/create
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:reactions*
     */
    notes___reactions___create: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            noteId: string,
            reaction: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * notes/reactions/delete
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:reactions*
     */
    notes___reactions___delete: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            noteId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * notes/renotes
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    notes___renotes: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            noteId: string,
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Note"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * notes/replies
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    notes___replies: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            noteId: string,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
            /** @default 10 */
            limit?: number,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Note"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * notes/search-by-tag
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    "notes___search-by-tag": {
      requestBody: {
        content: {
          "application/json": {
            /** @default null */
            reply?: boolean | null,
            /** @default null */
            renote?: boolean | null,
            /**
             * @description Only show notes that have attached files.
             * @default false
             */
            withFiles?: boolean,
            /** @default null */
            poll?: boolean | null,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
            /** @default 10 */
            limit?: number,
            tag?: string,
            /** @description The outer arrays are chained with OR, the inner arrays are chained with AND. */
            query?: string[][],
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Note"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * notes/search
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    notes___search: {
      requestBody: {
        content: {
          "application/json": {
            query: string,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
            /** @default 10 */
            limit?: number,
            /** @default 0 */
            offset?: number,
            /** @description The local host is represented with `.`. */
            host?: string,
            /**
             * Format: misskey:id
             * @default null
             */
            userId?: string | null,
            /**
             * Format: misskey:id
             * @default null
             */
            channelId?: string | null,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Note"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * notes/show
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    notes___show: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            noteId: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Note"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * notes/state
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    notes___state: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            noteId: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              isFavorited: boolean,
              isMutedThread: boolean,
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * notes/thread-muting/create
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    "notes___thread-muting___create": {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            noteId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * notes/thread-muting/delete
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    "notes___thread-muting___delete": {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            noteId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * notes/timeline
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    notes___timeline: {
      requestBody: {
        content: {
          "application/json": {
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
            sinceDate?: number,
            untilDate?: number,
            /** @default false */
            allowPartial?: boolean,
            /** @default true */
            includeMyRenotes?: boolean,
            /** @default true */
            includeRenotedMyNotes?: boolean,
            /** @default true */
            includeLocalRenotes?: boolean,
            /** @default false */
            withFiles?: boolean,
            /** @default true */
            withRenotes?: boolean,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Note"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * notes/translate
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    notes___translate: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            noteId: string,
            targetLang: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              sourceLang: string,
              text: string,
            },
          },
        },
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * notes/unrenote
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:notes*
     */
    notes___unrenote: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            noteId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * notes/user-list-timeline
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    "notes___user-list-timeline": {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            listId: string,
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
            sinceDate?: number,
            untilDate?: number,
            /** @default false */
            allowPartial?: boolean,
            /** @default true */
            includeMyRenotes?: boolean,
            /** @default true */
            includeRenotedMyNotes?: boolean,
            /** @default true */
            includeLocalRenotes?: boolean,
            /** @default true */
            withRenotes?: boolean,
            /**
             * @description Only show notes that have attached files.
             * @default false
             */
            withFiles?: boolean,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Note"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * notifications/create
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:notifications*
     */
    notifications___create: {
      requestBody: {
        content: {
          "application/json": {
            body: string,
            header?: string | null,
            icon?: string | null,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * notifications/flush
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:notifications*
     */
    notifications___flush: {
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * notifications/mark-all-as-read
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:notifications*
     */
    "notifications___mark-all-as-read": {
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * notifications/test-notification
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:notifications*
     */
    "notifications___test-notification": {
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * page-push
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    "page-push": {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            pageId: string,
            event: string,
            var?: unknown,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * pages/create
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:pages*
     */
    pages___create: {
      requestBody: {
        content: {
          "application/json": {
            title: string,
            name: string,
            summary?: string | null,
            content: {
                [key: string]: unknown,
              }[],
            variables: {
                [key: string]: unknown,
              }[],
            script: string,
            /** Format: misskey:id */
            eyeCatchingImageId?: string | null,
            /**
             * @default sans-serif
             * @enum {string}
             */
            font?: "serif" | "sans-serif",
            /** @default false */
            alignCenter?: boolean,
            /** @default false */
            hideTitleWhenPinned?: boolean,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Page"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * pages/delete
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:pages*
     */
    pages___delete: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            pageId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * pages/featured
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    pages___featured: {
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Page"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * pages/like
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:page-likes*
     */
    pages___like: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            pageId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * pages/show
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    pages___show: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            pageId?: string,
            name?: string,
            username?: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Page"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * pages/unlike
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:page-likes*
     */
    pages___unlike: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            pageId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * pages/update
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:pages*
     */
    pages___update: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            pageId: string,
            title?: string,
            name?: string,
            summary?: string | null,
            content?: {
                [key: string]: unknown,
              }[],
            variables?: {
                [key: string]: unknown,
              }[],
            script?: string,
            /** Format: misskey:id */
            eyeCatchingImageId?: string | null,
            /** @enum {string} */
            font?: "serif" | "sans-serif",
            alignCenter?: boolean,
            hideTitleWhenPinned?: boolean,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * flash/create
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:flash*
     */
    flash___create: {
      requestBody: {
        content: {
          "application/json": {
            title: string,
            summary: string,
            script: string,
            permissions: string[],
            /**
             * @default public
             * @enum {string}
             */
            visibility?: "public" | "private",
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Flash"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * flash/delete
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:flash*
     */
    flash___delete: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            flashId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * flash/featured
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    flash___featured: {
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Flash"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * flash/like
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:flash-likes*
     */
    flash___like: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            flashId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * flash/show
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    flash___show: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            flashId: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Flash"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * flash/unlike
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:flash-likes*
     */
    flash___unlike: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            flashId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * flash/update
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:flash*
     */
    flash___update: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            flashId: string,
            title?: string,
            summary?: string,
            script?: string,
            permissions?: string[],
            /** @enum {string} */
            visibility?: "public" | "private",
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * flash/my
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:flash*
     */
    flash___my: {
      requestBody: {
        content: {
          "application/json": {
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Flash"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * flash/my-likes
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:flash-likes*
     */
    "flash___my-likes": {
      requestBody: {
        content: {
          "application/json": {
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
                /** Format: id */
                id: string,
                flash: components["schemas"]["Flash"],
              }[],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * ping
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    ping: {
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              pong: number,
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * pinned-users
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    "pinned-users": {
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["UserDetailed"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * promo/read
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    promo___read: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            noteId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * roles/list
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    roles___list: {
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Role"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * roles/show
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    roles___show: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            roleId: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Role"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * roles/users
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    roles___users: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            roleId: string,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
            /** @default 10 */
            limit?: number,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
                /** Format: misskey:id */
                id: string,
                user: components["schemas"]["UserDetailed"],
              }[],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * roles/notes
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    roles___notes: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            roleId: string,
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
            sinceDate?: number,
            untilDate?: number,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Note"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * request-reset-password
     * @description Request a users password to be reset.
     *
     * **Credential required**: *No*
     */
    "request-reset-password": {
      requestBody: {
        content: {
          "application/json": {
            username: string,
            email: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * reset-db
     * @description Only available when running with <code>NODE_ENV=testing</code>. Reset the database and flush Redis.
     *
     * **Credential required**: *No*
     */
    "reset-db": {
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * reset-password
     * @description Complete the password reset that was previously requested.
     *
     * **Credential required**: *No*
     */
    "reset-password": {
      requestBody: {
        content: {
          "application/json": {
            token: string,
            password: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * server-info
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    "server-info": {
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              machine: string,
              cpu: {
                model: string,
                cores: number,
              },
              mem: {
                total: number,
              },
              fs: {
                total: number,
                used: number,
              },
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * stats
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    stats: {
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              notesCount: number,
              originalNotesCount: number,
              usersCount: number,
              originalUsersCount: number,
              instances: number,
              driveUsageLocal: number,
              driveUsageRemote: number,
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * sw/show-registration
     * @description Check push notification registration exists.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    "sw___show-registration": {
      requestBody: {
        content: {
          "application/json": {
            endpoint: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              userId: string,
              endpoint: string,
              sendReadMessage: boolean,
            } | null,
          },
        },
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * sw/update-registration
     * @description Update push notification registration.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    "sw___update-registration": {
      requestBody: {
        content: {
          "application/json": {
            endpoint: string,
            sendReadMessage?: boolean,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              userId: string,
              endpoint: string,
              sendReadMessage: boolean,
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * sw/register
     * @description Register to receive push notifications.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    sw___register: {
      requestBody: {
        content: {
          "application/json": {
            endpoint: string,
            auth: string,
            publickey: string,
            /** @default false */
            sendReadMessage?: boolean,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              /** @enum {string} */
              state?: "already-subscribed" | "subscribed",
              key: string | null,
              userId: string,
              endpoint: string,
              sendReadMessage: boolean,
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * sw/unregister
     * @description Unregister from receiving push notifications.
     *
     * **Credential required**: *No*
     */
    sw___unregister: {
      requestBody: {
        content: {
          "application/json": {
            endpoint: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * test
     * @description Endpoint for testing input validation.
     *
     * **Credential required**: *No*
     */
    test: {
      requestBody: {
        content: {
          "application/json": {
            required: boolean,
            string?: string,
            /** @default hello */
            default?: string,
            /** @default hello */
            nullableDefault?: string | null,
            /** Format: misskey:id */
            id?: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              /** Format: misskey:id */
              id?: string,
              required: boolean,
              string?: string,
              default?: string,
              /** @default hello */
              nullableDefault?: string | null,
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * username/available
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    username___available: {
      requestBody: {
        content: {
          "application/json": {
            username: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              available: boolean,
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * users
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    users: {
      requestBody: {
        content: {
          "application/json": {
            /** @default 10 */
            limit?: number,
            /** @default 0 */
            offset?: number,
            /** @enum {string} */
            sort?: "+follower" | "-follower" | "+createdAt" | "-createdAt" | "+updatedAt" | "-updatedAt",
            /**
             * @default all
             * @enum {string}
             */
            state?: "all" | "alive",
            /**
             * @default local
             * @enum {string}
             */
            origin?: "combined" | "local" | "remote",
            /**
             * @description The local host is represented with `null`.
             * @default null
             */
            hostname?: string | null,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["UserDetailed"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * users/clips
     * @description Show all clips this user owns.
     *
     * **Credential required**: *No*
     */
    users___clips: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            userId: string,
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Clip"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * users/followers
     * @description Show everyone that follows this user.
     *
     * **Credential required**: *No*
     */
    users___followers: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            userId?: string,
            username?: string,
            /** @description The local host is represented with `null`. */
            host?: string | null,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Following"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * users/following
     * @description Show everyone that this user is following.
     *
     * **Credential required**: *No*
     */
    users___following: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            userId?: string,
            username?: string,
            /** @description The local host is represented with `null`. */
            host?: string | null,
            birthday?: string | null,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Following"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * users/gallery/posts
     * @description Show all gallery posts by the given user.
     *
     * **Credential required**: *No*
     */
    users___gallery___posts: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            userId: string,
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["GalleryPost"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * users/get-frequently-replied-users
     * @description Get a list of other users that the specified user frequently replies to.
     *
     * **Credential required**: *No*
     */
    "users___get-frequently-replied-users": {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            userId: string,
            /** @default 10 */
            limit?: number,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
                user: components["schemas"]["UserDetailed"],
                weight: number,
              }[],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * users/featured-notes
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    "users___featured-notes": {
      requestBody: {
        content: {
          "application/json": {
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            untilId?: string,
            /** Format: misskey:id */
            userId: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Note"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * users/lists/create
     * @description Create a new list of users.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    users___lists___create: {
      requestBody: {
        content: {
          "application/json": {
            name: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["UserList"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * users/lists/delete
     * @description Delete an existing list of users.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    users___lists___delete: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            listId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * users/lists/list
     * @description Show all lists that the authenticated user has created.
     *
     * **Credential required**: *No* / **Permission**: *read:account*
     */
    users___lists___list: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            userId?: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["UserList"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * users/lists/pull
     * @description Remove a user from a list.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    users___lists___pull: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            listId: string,
            /** Format: misskey:id */
            userId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * users/lists/push
     * @description Add a user to an existing list.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    users___lists___push: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            listId: string,
            /** Format: misskey:id */
            userId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * users/lists/show
     * @description Show the properties of a list.
     *
     * **Credential required**: *No* / **Permission**: *read:account*
     */
    users___lists___show: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            listId: string,
            /** @default false */
            forPublic?: boolean,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["UserList"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * users/lists/favorite
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    users___lists___favorite: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            listId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * users/lists/unfavorite
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    users___lists___unfavorite: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            listId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * users/lists/update
     * @description Update the properties of a list.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    users___lists___update: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            listId: string,
            name?: string,
            isPublic?: boolean,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["UserList"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * users/lists/create-from-public
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    "users___lists___create-from-public": {
      requestBody: {
        content: {
          "application/json": {
            name: string,
            /** Format: misskey:id */
            listId: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["UserList"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * users/lists/update-membership
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    "users___lists___update-membership": {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            listId: string,
            /** Format: misskey:id */
            userId: string,
            withReplies?: boolean,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * users/lists/get-memberships
     * @description No description provided.
     *
     * **Credential required**: *No* / **Permission**: *read:account*
     */
    "users___lists___get-memberships": {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            listId: string,
            /** @default false */
            forPublic?: boolean,
            /** @default 30 */
            limit?: number,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
                /** Format: misskey:id */
                id: string,
                /** Format: date-time */
                createdAt: string,
                /** Format: misskey:id */
                userId: string,
                user: components["schemas"]["UserLite"],
                withReplies: boolean,
              }[],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * users/notes
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    users___notes: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            userId: string,
            /** @default false */
            withReplies?: boolean,
            /** @default true */
            withRenotes?: boolean,
            /** @default false */
            withChannelNotes?: boolean,
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
            sinceDate?: number,
            untilDate?: number,
            /** @default false */
            allowPartial?: boolean,
            /** @default false */
            withFiles?: boolean,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Note"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * users/pages
     * @description Show all pages this user created.
     *
     * **Credential required**: *No*
     */
    users___pages: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            userId: string,
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Page"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * users/flashs
     * @description Show all flashs this user created.
     *
     * **Credential required**: *No*
     */
    users___flashs: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            userId: string,
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["Flash"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * users/reactions
     * @description Show all reactions this user made.
     *
     * **Credential required**: *No*
     */
    users___reactions: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            userId: string,
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
            sinceDate?: number,
            untilDate?: number,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["NoteReaction"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * users/recommendation
     * @description Show users that the authenticated user might be interested to follow.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    users___recommendation: {
      requestBody: {
        content: {
          "application/json": {
            /** @default 10 */
            limit?: number,
            /** @default 0 */
            offset?: number,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["UserDetailed"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * users/relation
     * @description Show the different kinds of relations between the authenticated user and the specified user(s).
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    users___relation: {
      requestBody: {
        content: {
          "application/json": {
            userId: string | string[],
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": OneOf<[{
              /** Format: id */
              id: string,
              isFollowing: boolean,
              hasPendingFollowRequestFromYou: boolean,
              hasPendingFollowRequestToYou: boolean,
              isFollowed: boolean,
              isBlocking: boolean,
              isBlocked: boolean,
              isMuted: boolean,
              isRenoteMuted: boolean,
            }, {
                /** Format: id */
                id: string,
                isFollowing: boolean,
                hasPendingFollowRequestFromYou: boolean,
                hasPendingFollowRequestToYou: boolean,
                isFollowed: boolean,
                isBlocking: boolean,
                isBlocked: boolean,
                isMuted: boolean,
                isRenoteMuted: boolean,
              }[]]>,
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * users/report-abuse
     * @description File a report.
     *
     * **Credential required**: *Yes* / **Permission**: *write:report-abuse*
     */
    "users___report-abuse": {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            userId: string,
            comment: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * users/search-by-username-and-host
     * @description Search for a user by username and/or host.
     *
     * **Credential required**: *No*
     */
    "users___search-by-username-and-host": {
      requestBody: {
        content: {
          "application/json": {
            /** @default 10 */
            limit?: number,
            /** @default true */
            detail?: boolean,
            username?: string | null,
            host?: string | null,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["User"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * users/search
     * @description Search for users.
     *
     * **Credential required**: *No*
     */
    users___search: {
      requestBody: {
        content: {
          "application/json": {
            query: string,
            /** @default 0 */
            offset?: number,
            /** @default 10 */
            limit?: number,
            /**
             * @default combined
             * @enum {string}
             */
            origin?: "local" | "remote" | "combined",
            /** @default true */
            detail?: boolean,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["User"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * users/show
     * @description Show the properties of a user.
     *
     * **Credential required**: *No*
     */
    users___show: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            userId?: string,
            userIds?: string[],
            username?: string,
            /** @description The local host is represented with `null`. */
            host?: string | null,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["UserDetailed"] | components["schemas"]["UserDetailed"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * users/achievements
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    users___achievements: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            userId: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
                name: string,
                unlockedAt: number,
              }[],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * users/update-memo
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    "users___update-memo": {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            userId: string,
            /** @description A personal memo for the target user. If null or empty, delete the memo. */
            memo: string | null,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * fetch-rss
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    "fetch-rss": {
      requestBody: {
        content: {
          "application/json": {
            url: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              image?: {
                link?: string,
                url: string,
                title?: string,
              },
              paginationLinks?: {
                self?: string,
                first?: string,
                next?: string,
                last?: string,
                prev?: string,
              },
              link?: string,
              title?: string,
              items: {
                  link?: string,
                  guid?: string,
                  title?: string,
                  pubDate?: string,
                  creator?: string,
                  summary?: string,
                  content?: string,
                  isoDate?: string,
                  categories?: string[],
                  contentSnippet?: string,
                  enclosure?: {
                    url: string,
                    length?: number,
                    type?: string,
                  },
                }[],
              feedUrl?: string,
              description?: string,
              itunes?: {
                image?: string,
                owner?: {
                  name?: string,
                  email?: string,
                },
                author?: string,
                summary?: string,
                explicit?: string,
                categories?: string[],
                keywords?: string[],
                [key: string]: unknown,
              },
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * fetch-external-resources
     * @description No description provided.
     *
     * **Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.
     * **Credential required**: *Yes*
     */
    "fetch-external-resources": {
      requestBody: {
        content: {
          "application/json": {
            url: string,
            hash: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              type: string,
              data: string,
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * retention
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    retention: {
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
                /** Format: date-time */
                createdAt: string,
                users: number,
                data: {
                  [key: string]: number,
                },
              }[],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * bubble-game/register
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    "bubble-game___register": {
      requestBody: {
        content: {
          "application/json": {
            score: number,
            seed: string,
            logs: number[][],
            gameMode: string,
            gameVersion: number,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description To many requests */
        429: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * bubble-game/ranking
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    "bubble-game___ranking": {
      requestBody: {
        content: {
          "application/json": {
            gameMode: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
                /** Format: misskey:id */
                id: string,
                score: number,
                user?: components["schemas"]["UserLite"],
              }[],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * reversi/cancel-match
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    "reversi___cancel-match": {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            userId?: string | null,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * reversi/games
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    reversi___games: {
      requestBody: {
        content: {
          "application/json": {
            /** @default 10 */
            limit?: number,
            /** Format: misskey:id */
            sinceId?: string,
            /** Format: misskey:id */
            untilId?: string,
            /** @default false */
            my?: boolean,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["ReversiGameLite"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * reversi/match
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    reversi___match: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            userId?: string | null,
            /** @default false */
            noIrregularRules?: boolean,
            /** @default false */
            multiple?: boolean,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["ReversiGameDetailed"],
          },
        },
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * reversi/invitations
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *read:account*
     */
    reversi___invitations: {
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["UserLite"][],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * reversi/show-game
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    "reversi___show-game": {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            gameId: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": components["schemas"]["ReversiGameDetailed"],
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * reversi/surrender
     * @description No description provided.
     *
     * **Credential required**: *Yes* / **Permission**: *write:account*
     */
    reversi___surrender: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            gameId: string,
          },
        },
      },
      responses: {
        /** @description OK (without any results) */
        204: {
          content: never,
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
    /**
     * reversi/verify
     * @description No description provided.
     *
     * **Credential required**: *No*
     */
    reversi___verify: {
      requestBody: {
        content: {
          "application/json": {
            /** Format: misskey:id */
            gameId: string,
            crc32: string,
          },
        },
      },
      responses: {
        /** @description OK (with results) */
        200: {
          content: {
            "application/json": {
              desynced: boolean,
              game?: components["schemas"]["ReversiGameDetailed"] | null,
            },
          },
        },
        /** @description Client error */
        400: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Authentication error */
        401: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Forbidden error */
        403: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description I"m Ai */
        418: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
        /** @description Internal server error */
        500: {
          content: {
            "application/json": components["schemas"]["Error"],
          },
        },
      },
    },
}