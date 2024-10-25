/*
 * Janssen Project software is available under the MIT License (2008). See http://opensource.org/licenses/MIT for full text.
 *
 * Copyright (c) 2020, Janssen Project
 */

package io.jans.ca.plugin.adminui;

import com.fasterxml.jackson.databind.DeserializationFeature;
import com.fasterxml.jackson.databind.ObjectMapper;

import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;


import org.testng.ITestContext;
import org.testng.annotations.AfterSuite;
import org.testng.annotations.BeforeSuite;

import java.nio.file.Paths;
import java.util.Map;


public class AdminUiBaseTest {
	
     protected Logger logger = LogManager.getLogger(getClass());
    protected ObjectMapper mapper = new ObjectMapper();

   @BeforeSuite
    public void initTestSuite(ITestContext context) throws Exception {

        logger.info("Invoked initTestSuite of '{}'", context.getCurrentXmlTest().getName());
        if (client == null) {
            setupClient(context.getSuite().getXmlSuite().getParameters());
            mapper.disable(DeserializationFeature.FAIL_ON_UNKNOWN_PROPERTIES);
        }
        
    }

    @AfterSuite
    public void finalize() {
        client.close();
    }

    private void setupClient(Map<String, String> params) throws Exception {

        logger.info("Initializing client...");
        

    }


}
