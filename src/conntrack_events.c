#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <errno.h>

#include <libnetfilter_conntrack/libnetfilter_conntrack.h>

typedef void (*rust_callback)(void *flow_statics, char *buf);
void *flow_statics;
rust_callback cb;

static int event_cb(enum nf_conntrack_msg_type type,
					struct nf_conntrack *ct,
					void *data __attribute__((unused)))
{
	char buf[1024];
	nfct_snprintf(buf, sizeof(buf), ct, type, NFCT_O_PLAIN, NFCT_OF_TIMESTAMP | NFCT_OF_SHOW_LAYER3);
	cb(flow_statics, buf);
	return NFCT_CB_CONTINUE;
}

int register_event(void *callback_target, rust_callback callback)
{
	flow_statics = callback_target;
	cb = callback;
	int ret;
	struct nfct_handle *h;

	h = nfct_open(CONNTRACK, NF_NETLINK_CONNTRACK_DESTROY | NF_NETLINK_CONNTRACK_EXP_DESTROY);
	if (!h)
	{
		perror("nfct_open");
		return 0;
	}

	nfct_callback_register(h, NFCT_T_DESTROY, event_cb, NULL);

	ret = nfct_catch(h);

	if (ret == -1)
	{
		printf("conntrack events error, (%d)(%s)\n", ret, strerror(errno));
	}

	nfct_close(h);

	return ret;
}
