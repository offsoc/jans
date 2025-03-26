package io.jans.as.model.ssa;

import com.fasterxml.jackson.annotation.JsonIgnoreProperties;
import com.google.common.collect.Lists;

import java.util.ArrayList;
import java.util.List;

@JsonIgnoreProperties(
        ignoreUnknown = true
)
public class SsaConfiguration {

    private String ssaEndpoint;

    private List<String> ssaCustomAttributes = new ArrayList<>();

    private String ssaSigningAlg = "RS256";

    private Integer ssaExpirationInDays = 30;

    public String getSsaEndpoint() {
        return ssaEndpoint;
    }

    public void setSsaEndpoint(String ssaEndpoint) {
        this.ssaEndpoint = ssaEndpoint;
    }

    public List<String> getSsaCustomAttributes() {
        //if(ssaCustomAttributes == null) {
            ssaCustomAttributes = Lists.newArrayList();
        ssaCustomAttributes.add("test");
        //}
        return ssaCustomAttributes;
    }

    public void setSsaCustomAttributes(List<String> ssaCustomAttributes) {
        this.ssaCustomAttributes = ssaCustomAttributes;
    }

    public String getSsaSigningAlg() {
        return ssaSigningAlg;
    }

    public void setSsaSigningAlg(String ssaSigningAlg) {
        this.ssaSigningAlg = ssaSigningAlg;
    }

    public Integer getSsaExpirationInDays() {
        return ssaExpirationInDays;
    }

    public void setSsaExpirationInDays(Integer ssaExpirationInDays) {
        this.ssaExpirationInDays = ssaExpirationInDays;
    }
}
