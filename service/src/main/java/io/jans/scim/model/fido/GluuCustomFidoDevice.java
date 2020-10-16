/*
 * Janssen Project software is available under the MIT License (2008). See http://opensource.org/licenses/MIT for full text.
 *
 * Copyright (c) 2020, Janssen Project
 */

package io.jans.scim.model.fido;

import io.jans.orm.model.base.Entry;
import io.jans.orm.annotation.AttributeName;
import io.jans.orm.annotation.DataEntry;
import io.jans.orm.annotation.ObjectClass;

/**
 * @author Val Pecaoco
 */
@DataEntry(sortBy = { "id" })
@ObjectClass(value = "oxDeviceRegistration")
public class GluuCustomFidoDevice extends Entry {

	/**
	 * 
	 */
	private static final long serialVersionUID = 4463359164739925541L;

	@AttributeName(name = "jsId", ignoreDuringUpdate = true)
	private String id;

	@AttributeName(name = "creationDate", ignoreDuringUpdate = true)
	private String creationDate;

	@AttributeName(name = "jsApp", ignoreDuringUpdate = true)
	private String application;

	@AttributeName(name = "jsCounter", ignoreDuringUpdate = true)
	private String counter;

	@AttributeName(name = "jsDeviceData", ignoreDuringUpdate = true)
	private String deviceData;

	@AttributeName(name = "jsDeviceHashCode", ignoreDuringUpdate = true)
	private String deviceHashCode;

	@AttributeName(name = "jsDeviceKeyHandle", ignoreDuringUpdate = true)
	private String deviceKeyHandle;

	@AttributeName(name = "jsDeviceRegistrationConf", ignoreDuringUpdate = true)
	private String deviceRegistrationConf;

	@AttributeName(name = "jsLastAccessTime", ignoreDuringUpdate = true)
	private String lastAccessTime;

	@AttributeName(name = "jsStatus")
	private String status;

	@AttributeName(name = "displayName")
	private String displayName;

	@AttributeName(name = "description")
	private String description;

	@AttributeName(name = "jsNickName")
	private String nickname;

	@AttributeName(name = "excludeMetaLastMod")
	private String metaLastModified;

	@AttributeName(name = "excludeMetaLocation")
	private String metaLocation;

	@AttributeName(name = "excludeMetaVer")
	private String metaVersion;

	public String getId() {
		return id;
	}

	public void setId(String id) {
		this.id = id;
	}

	public String getCreationDate() {
		return creationDate;
	}

	public void setCreationDate(String creationDate) {
		this.creationDate = creationDate;
	}

	public String getApplication() {
		return application;
	}

	public void setApplication(String application) {
		this.application = application;
	}

	public String getCounter() {
		return counter;
	}

	public void setCounter(String counter) {
		this.counter = counter;
	}

	public String getDeviceData() {
		return deviceData;
	}

	public void setDeviceData(String deviceData) {
		this.deviceData = deviceData;
	}

	public String getDeviceHashCode() {
		return deviceHashCode;
	}

	public void setDeviceHashCode(String deviceHashCode) {
		this.deviceHashCode = deviceHashCode;
	}

	public String getDeviceKeyHandle() {
		return deviceKeyHandle;
	}

	public void setDeviceKeyHandle(String deviceKeyHandle) {
		this.deviceKeyHandle = deviceKeyHandle;
	}

	public String getDeviceRegistrationConf() {
		return deviceRegistrationConf;
	}

	public void setDeviceRegistrationConf(String deviceRegistrationConf) {
		this.deviceRegistrationConf = deviceRegistrationConf;
	}

	public String getLastAccessTime() {
		return lastAccessTime;
	}

	public void setLastAccessTime(String lastAccessTime) {
		this.lastAccessTime = lastAccessTime;
	}

	public String getStatus() {
		return status;
	}

	public void setStatus(String status) {
		this.status = status;
	}

	public String getDisplayName() {
		return displayName;
	}

	public void setDisplayName(String displayName) {
		this.displayName = displayName;
	}

	public String getDescription() {
		return description;
	}

	public void setDescription(String description) {
		this.description = description;
	}

	public String getNickname() {
		return nickname;
	}

	public void setNickname(String nickname) {
		this.nickname = nickname;
	}

	public String getMetaLastModified() {
		return metaLastModified;
	}

	public void setMetaLastModified(String metaLastModified) {
		this.metaLastModified = metaLastModified;
	}

	public String getMetaLocation() {
		return metaLocation;
	}

	public void setMetaLocation(String metaLocation) {
		this.metaLocation = metaLocation;
	}

	public String getMetaVersion() {
		return metaVersion;
	}

	public void setMetaVersion(String metaVersion) {
		this.metaVersion = metaVersion;
	}
}
