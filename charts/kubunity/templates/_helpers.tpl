{{/*
Expand the name of the chart.
*/}}
{{- define "kubunity.name" -}}
{{- default .Chart.Name .Values.nameOverride | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Create a default fully qualified app name.
*/}}
{{- define "kubunity.fullname" -}}
{{- if .Values.fullnameOverride }}
{{- .Values.fullnameOverride | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- $name := default .Chart.Name .Values.nameOverride }}
{{- if contains $name .Release.Name }}
{{- .Release.Name | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- printf "%s-%s" .Release.Name $name | trunc 63 | trimSuffix "-" }}
{{- end }}
{{- end }}
{{- end }}

{{/*
Create chart name and version as used by the chart label.
*/}}
{{- define "kubunity.chart" -}}
{{- printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Common labels
*/}}
{{- define "kubunity.labels" -}}
helm.sh/chart: {{ include "kubunity.chart" . }}
{{ include "kubunity.selectorLabels" . }}
{{- if .Chart.AppVersion }}
app.kubernetes.io/version: {{ .Chart.AppVersion | quote }}
{{- end }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
kubunity.io/cluster: {{ .Values.cluster.name | quote }}
kubunity.io/environment: {{ .Values.cluster.environment | quote }}
kubunity.io/profile: {{ .Values.cluster.profile | quote }}
{{- end }}

{{/*
Selector labels
*/}}
{{- define "kubunity.selectorLabels" -}}
app.kubernetes.io/name: {{ include "kubunity.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}
