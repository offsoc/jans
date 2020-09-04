package org.gluu.configapi.rest.resource;

import org.gluu.oxauth.model.configuration.AppConfiguration;
import org.gluu.configapi.filters.ProtectedApi;
import org.gluu.configapi.util.ApiConstants;
import org.gluu.configapi.util.Jackson;
import org.gluu.oxtrust.service.JsonConfigurationService;
import org.json.JSONObject;
import org.slf4j.Logger;

import javax.inject.Inject;
import javax.validation.constraints.NotNull;
import javax.ws.rs.*;
import javax.ws.rs.core.MediaType;
import javax.ws.rs.core.Response;

import java.io.IOException;

@Path(ApiConstants.BASE_API_URL + ApiConstants.CONFIG + ApiConstants.OXAUTH)
@Produces(MediaType.APPLICATION_JSON)
@Consumes(MediaType.APPLICATION_JSON)
public class ConfigResource extends BaseResource {

	@Inject
	Logger log;

	@Inject
	JsonConfigurationService jsonConfigurationService;

	@GET
    @ProtectedApi(scopes = {READ_ACCESS})
	public Response getAppConfiguration() throws IOException {		
		AppConfiguration appConfiguration = this.jsonConfigurationService.getOxauthAppConfiguration();
		JSONObject json = new JSONObject(appConfiguration);
		return Response.ok(appConfiguration).build();
	
	}

    @PATCH
    @Consumes(MediaType.APPLICATION_JSON_PATCH_JSON)
    @ProtectedApi(scopes = {WRITE_ACCESS})
    public Response patchAppConfigurationProperty(@NotNull String requestString) throws Exception{
        log.trace("=======================================================================");
        log.trace("\n\n requestString = " + requestString + "\n\n");
    
        AppConfiguration appConfiguration = this.jsonConfigurationService.getOxauthAppConfiguration();

        final JSONObject jsonBefore = new JSONObject(appConfiguration);
        log.trace("\n\n appConfiguration_before = " + jsonBefore + "\n\n");

        appConfiguration = Jackson.applyPatch(requestString, appConfiguration);

        JSONObject jsonAfter = new JSONObject(appConfiguration);
        log.trace("\n\n appConfiguration_after = " + jsonAfter + "\n\n");
        log.trace("=======================================================================");

        jsonConfigurationService.saveOxAuthAppConfiguration(appConfiguration);

        return Response.ok(appConfiguration).build();
     
    }
}
